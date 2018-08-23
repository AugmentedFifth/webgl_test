use error::Error;
use geometry;
use mains;
use map;
use na;
use physics;
use std::{f32::consts::FRAC_PI_2, sync::Mutex};
use webgl;

struct GlState {
    terrain: TerrainRendering,
    skybox:  SkyboxRendering,
}

struct TerrainRendering {
    program:                   webgl::WebGLProgram,
    vao:                       webgl::WebGLVertexArrayObject,
    world_uni_loc:             webgl::WebGLUniformLocation,
    world_view_proj_uni_loc:   webgl::WebGLUniformLocation,
    reverse_light_dir_uni_loc: webgl::WebGLUniformLocation,
    color_uni_loc:             webgl::WebGLUniformLocation,
    displacement_uni_loc:      webgl::WebGLUniformLocation,
}

struct SkyboxRendering {
    program:         webgl::WebGLProgram,
    vao:             webgl::WebGLVertexArrayObject,
    proj_uni_loc:    webgl::WebGLUniformLocation,
    view_uni_loc:    webgl::WebGLUniformLocation,
    texture_uni_loc: webgl::WebGLUniformLocation,
    cube_map:        webgl::WebGLTexture,
    textures_loaded: bool,
}

lazy_static! {
    static ref GL_STATE: Mutex<Option<GlState>> = Mutex::new(None);
}

const VERTEX_SHADER_SRC: &str = r#"#version 300 es

in vec4 a_position;
in vec3 a_normal;

uniform mat4 u_worldViewProjection;
uniform mat4 u_world;
uniform vec3 u_displacement;

out vec3 v_normal;

void main() {
    vec4 displaced = a_position + vec4(u_displacement, 0.0);
    gl_Position = u_worldViewProjection * displaced;

    v_normal = mat3(u_world) * a_normal;
}
"#;

const FRAGMENT_SHADER_SRC: &str = r#"#version 300 es

precision mediump float;

in vec3 v_normal;

uniform vec3 u_reverseLightDirection;
uniform vec3 u_color;

out vec4 outColor;

void main() {
    float light = dot(v_normal, u_reverseLightDirection);

    outColor = vec4(u_color * light, 1.0);
}
"#;

const SKYBOX_VERTEX_SHADER_SRC: &str = r#"#version 300 es

in vec3 a_position;

uniform mat4 u_projection;
uniform mat4 u_view;

out vec3 texCoords;

void main() {
    texCoords = a_position;

    gl_Position = (u_projection * u_view * vec4(a_position, 1.0)).xyww;
}
"#;

const SKYBOX_FRAGMENT_SHADER_SRC: &str = r#"#version 300 es

precision mediump float;

in vec3 texCoords;

uniform samplerCube u_texture;

out vec4 fragmentColor;

void main() {
    fragmentColor = texture(u_texture, texCoords);
}
"#;

const SKYBOX_TEXTURE_INDEX: webgl::TextureIndex =
    webgl::TextureIndex::Texture0;

pub fn init() -> Result<(), Error> {
    let vertex_shader = webgl::create_shader(
        webgl::ShaderType::VertexShader,
        VERTEX_SHADER_SRC,
    ).ok_or_else(|| Error::Gl("Failed to create vertex shader".to_owned()))?;
    let fragment_shader = webgl::create_shader(
        webgl::ShaderType::FragmentShader,
        FRAGMENT_SHADER_SRC,
    ).ok_or_else(|| Error::Gl("Failed to create fragment shader".to_owned()))?;

    let program = webgl::create_program(&vertex_shader, &fragment_shader)
        .ok_or_else(|| Error::Gl("Failed to link GLSL program".to_owned()))?;

    let world_view_proj_uni_loc =
        webgl::get_uniform_location(&program, "u_worldViewProjection")
            .ok_or_else(|| {
                Error::Gl(
                    "There is no uniform with the name \
                     \"u_worldViewProjection\""
                        .to_owned(),
                )
            })?;
    let world_uni_loc = webgl::get_uniform_location(&program, "u_world")
        .ok_or_else(|| {
            Error::Gl(
                "There is no uniform with the name \"u_world\"".to_owned(),
            )
        })?;
    let color_uni_loc = webgl::get_uniform_location(&program, "u_color")
        .ok_or_else(|| {
            Error::Gl(
                "There is no uniform with the name \"u_color\"".to_owned(),
            )
        })?;
    let displacement_uni_loc = webgl::get_uniform_location(
        &program,
        "u_displacement",
    ).ok_or_else(|| {
        Error::Gl(
            "There is no uniform with the name \"u_displacement\"".to_owned(),
        )
    })?;
    let reverse_light_dir_uni_loc =
        webgl::get_uniform_location(&program, "u_reverseLightDirection")
            .ok_or_else(|| {
                Error::Gl(
                    "There is no uniform with the name \
                     \"u_reverseLightDirection\""
                        .to_owned(),
                )
            })?;
    ////////////////////////////////////////////////////////////////////////

    let position_attr_loc = webgl::get_attr_location(&program, "a_position");
    if position_attr_loc < 0 {
        return Err(Error::Gl(
            "There is no attribute with the name \"a_position\"".to_owned(),
        ));
    }
    let position_attr_loc = position_attr_loc as u32;
    let normal_attr_loc = webgl::get_attr_location(&program, "a_normal");
    if normal_attr_loc < 0 {
        return Err(Error::Gl(
            "There is no attribute with the name \"a_normal\"".to_owned(),
        ));
    }
    let normal_attr_loc = normal_attr_loc as u32;
    ////////////////////////////////////////////////////////////////////////

    let prism_vert_buffer = webgl::create_buffer();
    webgl::bind_buffer(webgl::BufferType::ArrayBuffer, &prism_vert_buffer);
    webgl::buffer_data_f32(
        webgl::BufferType::ArrayBuffer,
        geometry::HEXAGONAL_PRISM,
        webgl::UsageType::StaticDraw,
        0,
        None,
    );

    let prism_normal_buffer = webgl::create_buffer();
    webgl::bind_buffer(webgl::BufferType::ArrayBuffer, &prism_normal_buffer);
    webgl::buffer_data_f32(
        webgl::BufferType::ArrayBuffer,
        geometry::HEXAGONAL_PRISM_NORMALS,
        webgl::UsageType::StaticDraw,
        0,
        None,
    );
    ////////////////////////////////////////////////////////////////////////

    let vao = webgl::create_vertex_array();
    webgl::bind_vertex_array(&vao);
    webgl::bind_buffer(webgl::BufferType::ArrayBuffer, &prism_vert_buffer);
    webgl::enable_vertex_attr_array(position_attr_loc);
    webgl::vertex_attr_ptr(
        position_attr_loc,
        3,                      // Three components per iteration
        webgl::DataType::Float, // The data is `f32`s
        false,                  // Don't normalize to clip space
        0,                      // Stride (in bytes)
        0,                      // Offset (in bytes)
    );

    webgl::bind_buffer(webgl::BufferType::ArrayBuffer, &prism_normal_buffer);
    webgl::enable_vertex_attr_array(normal_attr_loc);
    webgl::vertex_attr_ptr(
        normal_attr_loc,
        3,                      // Three components per iteration
        webgl::DataType::Float, // The data is `f32`s
        false,                  // Do not normalize
        0,                      // Stride (in bytes)
        0,                      // Offset (in bytes)
    );
    ////////////////////////////////////////////////////////////////////////

    let terrain = TerrainRendering {
        program,
        vao,
        world_uni_loc,
        world_view_proj_uni_loc,
        reverse_light_dir_uni_loc,
        color_uni_loc,
        displacement_uni_loc,
    };

    let vertex_shader = webgl::create_shader(
        webgl::ShaderType::VertexShader,
        SKYBOX_VERTEX_SHADER_SRC,
    ).ok_or_else(|| Error::Gl("Failed to create vertex shader".to_owned()))?;
    let fragment_shader = webgl::create_shader(
        webgl::ShaderType::FragmentShader,
        SKYBOX_FRAGMENT_SHADER_SRC,
    ).ok_or_else(|| Error::Gl("Failed to create fragment shader".to_owned()))?;

    let program = webgl::create_program(&vertex_shader, &fragment_shader)
        .ok_or_else(|| Error::Gl("Failed to link GLSL program".to_owned()))?;
    webgl::use_program(&program);

    let proj_uni_loc = webgl::get_uniform_location(&program, "u_projection")
        .ok_or_else(|| {
        Error::Gl(
            "There is no uniform with the name \"u_projection\"".to_owned(),
        )
    })?;
    let view_uni_loc = webgl::get_uniform_location(&program, "u_view")
        .ok_or_else(|| {
            Error::Gl(
                "There is no uniform with the name \"u_view\"".to_owned(),
            )
        })?;
    let texture_uni_loc = webgl::get_uniform_location(&program, "u_texture")
        .ok_or_else(|| {
        Error::Gl("There is no uniform with the name \"u_texture\"".to_owned())
    })?;
    ////////////////////////////////////////////////////////////////////////

    let position_attr_loc = webgl::get_attr_location(&program, "a_position");
    if position_attr_loc < 0 {
        return Err(Error::Gl(
            "There is no attribute with the name \"a_position\"".to_owned(),
        ));
    }
    let position_attr_loc = position_attr_loc as u32;
    ////////////////////////////////////////////////////////////////////////

    let tex_pos_buffer = webgl::create_buffer();
    webgl::bind_buffer(webgl::BufferType::ArrayBuffer, &tex_pos_buffer);
    webgl::buffer_data_f32(
        webgl::BufferType::ArrayBuffer,
        &[
            -1.0, 1.0, -1.0, ////////////////
            -1.0, -1.0, -1.0, ////////////////
            1.0, -1.0, -1.0, ////////////////
            1.0, -1.0, -1.0, ////////////////
            1.0, 1.0, -1.0, ////////////////
            -1.0, 1.0, -1.0, ////////////////
            ////////////////////////////////////////////////////////////////
            -1.0, -1.0, 1.0, ////////////////
            -1.0, -1.0, -1.0, ////////////////
            -1.0, 1.0, -1.0, ////////////////
            -1.0, 1.0, -1.0, ////////////////
            -1.0, 1.0, 1.0, ////////////////
            -1.0, -1.0, 1.0, ////////////////
            ////////////////////////////////////////////////////////////////
            1.0, -1.0, -1.0, ////////////////
            1.0, -1.0, 1.0, ////////////////
            1.0, 1.0, 1.0, ////////////////
            1.0, 1.0, 1.0, ////////////////
            1.0, 1.0, -1.0, ////////////////
            1.0, -1.0, -1.0, ////////////////
            ////////////////////////////////////////////////////////////////
            -1.0, -1.0, 1.0, ////////////////
            -1.0, 1.0, 1.0, ////////////////
            1.0, 1.0, 1.0, ////////////////
            1.0, 1.0, 1.0, ////////////////
            1.0, -1.0, 1.0, ////////////////
            -1.0, -1.0, 1.0, ////////////////
            ////////////////////////////////////////////////////////////////
            -1.0, 1.0, -1.0, ////////////////
            1.0, 1.0, -1.0, ////////////////
            1.0, 1.0, 1.0, ////////////////
            1.0, 1.0, 1.0, ////////////////
            -1.0, 1.0, 1.0, ////////////////
            -1.0, 1.0, -1.0, ////////////////
            ////////////////////////////////////////////////////////////////
            -1.0, -1.0, -1.0, ////////////////
            -1.0, -1.0, 1.0, ////////////////
            1.0, -1.0, -1.0, ////////////////
            1.0, -1.0, -1.0, ////////////////
            -1.0, -1.0, 1.0, ////////////////
            1.0, -1.0, 1.0, ////////////////
        ],
        webgl::UsageType::StaticDraw,
        0,
        None,
    );
    ////////////////////////////////////////////////////////////////////////

    let vao = webgl::create_vertex_array();
    webgl::bind_vertex_array(&vao);
    webgl::bind_buffer(webgl::BufferType::ArrayBuffer, &tex_pos_buffer);
    webgl::enable_vertex_attr_array(position_attr_loc);
    webgl::vertex_attr_ptr(
        position_attr_loc,
        3,                      // Three components per iteration
        webgl::DataType::Float, // The data is `f32`s
        false,                  // Don't normalize to clip space
        0,                      // Stride (in bytes)
        0,                      // Offset (in bytes)
    );
    ////////////////////////////////////////////////////////////////////////

    let cube_map = webgl::create_texture();
    webgl::active_texture(SKYBOX_TEXTURE_INDEX);
    webgl::bind_texture(webgl::TextureTarget::TextureCubeMap, &cube_map);
    webgl::tex_parameteri(
        webgl::TextureTarget::TextureCubeMap,
        webgl::TextureParam::TextureMinFilter,
        webgl::NEAREST,
    );
    webgl::tex_parameteri(
        webgl::TextureTarget::TextureCubeMap,
        webgl::TextureParam::TextureMagFilter,
        webgl::NEAREST,
    );
    webgl::tex_parameteri(
        webgl::TextureTarget::TextureCubeMap,
        webgl::TextureParam::TextureWrapS,
        webgl::CLAMP_TO_EDGE,
    );
    webgl::tex_parameteri(
        webgl::TextureTarget::TextureCubeMap,
        webgl::TextureParam::TextureWrapT,
        webgl::CLAMP_TO_EDGE,
    );
    webgl::tex_parameteri(
        webgl::TextureTarget::TextureCubeMap,
        webgl::TextureParam::TextureWrapR,
        webgl::CLAMP_TO_EDGE,
    );
    webgl::pixel_storei(webgl::PixelStoreParam::UnpackAlignment, 1);
    webgl::uniform1i(&texture_uni_loc, SKYBOX_TEXTURE_INDEX.as_index() as i32);
    ////////////////////////////////////////////////////////////////////////

    let skybox = SkyboxRendering {
        program,
        vao,
        proj_uni_loc,
        view_uni_loc,
        texture_uni_loc,
        cube_map,
        textures_loaded: false,
    };

    *GL_STATE.lock().unwrap() = Some(GlState { terrain, skybox });
    ////////////////////////////////////////////////////////////////////////

    // Reset the canvas
    webgl::resize_canvas_to_display();
    webgl::reset_viewport();
    webgl::clear_color(0.0, 0.0, 0.0, 0.0);
    webgl::clear(webgl::COLOR_BUFFER_BIT | webgl::DEPTH_BUFFER_BIT);

    // Capabilities
    webgl::enable(webgl::Capability::CullFace);
    webgl::enable(webgl::Capability::DepthTest);
    //////////////////////////////////////////////////

    Ok(())
}

pub fn render() -> Result<(), Error> {
    // Retrieve GL state
    let mut gl_state_lock = GL_STATE.lock().unwrap();
    let gl_state = gl_state_lock.as_mut().ok_or_else(|| {
        Error::Logic("Did not call `init` before callng `render`".to_owned())
    })?;

    // Retrieve physical state
    let world = physics::WORLD.lock().unwrap();
    let player_body =
        world.rigid_body(*physics::PLAYER.lock().unwrap()).unwrap();

    // Retrieve player-specific state
    let player_state = mains::PLAYER_STATE.lock().unwrap();

    // Retrieve map state
    let map_state = map::MAP.lock().unwrap();

    // Reset the canvas
    webgl::resize_canvas_to_display();
    webgl::reset_viewport();
    webgl::clear_color(0.0, 0.0, 0.0, 0.0);
    webgl::clear(webgl::COLOR_BUFFER_BIT | webgl::DEPTH_BUFFER_BIT);

    // Compute transformation matrices
    let proj = na::Matrix4::new_perspective(
        webgl::get_canvas_width() / webgl::get_canvas_height(),
        0.875,
        1.0,
        2000.0,
    );
    let world =
        na::Matrix4::new_rotation(na::Vector3::new(-FRAC_PI_2, 0.0, 0.0));
    let player_com = player_body.center_of_mass();
    let player_orient = player_state.orient.unwrap();
    let view = na::Matrix4::look_at_rh(
        &player_com,
        &(player_com + player_orient),
        &na::Vector3::y(),
    ) * world;
    let view_rot_only =
        na::Rotation3::look_at_rh(&player_orient, &na::Vector3::y());
    let view_proj = proj * view;

    ////////////////////////////////////////////////////////////////////
    //////////////////////// Rendering terrain /////////////////////////
    ////////////////////////////////////////////////////////////////////

    // Tell it to use the terrain program (pair of shaders)
    webgl::use_program(&gl_state.terrain.program);
    // Bind the attribute/buffer set we want
    webgl::bind_vertex_array(&gl_state.terrain.vao);
    ////////////////////////////////////////////////////////////////////

    // Pass in the world matrix
    webgl::uniform_matrix4fv(
        &gl_state.terrain.world_uni_loc,
        world.as_slice(),
    );

    // Pass in the world view/projection matrix
    webgl::uniform_matrix4fv(
        &gl_state.terrain.world_view_proj_uni_loc,
        view_proj.as_slice(),
    );

    // Set the light direction
    let light_dir = na::Vector3::new(0.7, 1.0, 0.5).normalize();
    webgl::uniform3f(
        &gl_state.terrain.reverse_light_dir_uni_loc,
        light_dir[0],
        light_dir[1],
        light_dir[2],
    );

    let player_cube_coord =
        geometry::pixel_to_cube(player_com[0], player_com[2]);
    let player_hex = map_state.index_by_cube(player_cube_coord);
    for (hex, (x, y)) in player_hex
        .into_iter()
        .chain(map_state.iter_radial(player_cube_coord))
    {
        // Set the color
        let color = map::RgbColor::from_byte_color(hex.color);
        webgl::uniform3f(
            &gl_state.terrain.color_uni_loc,
            color.r(),
            color.g(),
            color.b(),
        );

        // Set the coordinates
        webgl::uniform3f(
            &gl_state.terrain.displacement_uni_loc,
            *x,
            *y,
            hex.height,
        );
        ////////////////////////////////////////////////////////////////////

        // Draw!
        webgl::draw_arrays(
            webgl::RenderingPrimitive::Triangles,
            0,
            geometry::HEXAGONAL_PRISM.len() as i32 / 3,
        );
        ////////////////////////////////////////////////////////////////////
    }

    ////////////////////////////////////////////////////////////////////
    ///////////////////////// Rendering skybox /////////////////////////
    ////////////////////////////////////////////////////////////////////

    // Tell it to use the skybox program (pair of shaders)
    webgl::use_program(&gl_state.skybox.program);
    // Bind the attribute/buffer set we want
    webgl::bind_vertex_array(&gl_state.skybox.vao);
    ////////////////////////////////////////////////////////////////////

    webgl::depth_func(webgl::DepthFunc::LEqual);

    webgl::uniform_matrix4fv(&gl_state.skybox.proj_uni_loc, proj.as_slice());
    webgl::uniform_matrix4fv(
        &gl_state.skybox.view_uni_loc,
        view_rot_only.to_homogeneous().as_slice(),
    );
    ////////////////////////////////////////////////////////////////////

    webgl::active_texture(SKYBOX_TEXTURE_INDEX);
    webgl::bind_texture(
        webgl::TextureTarget::TextureCubeMap,
        &gl_state.skybox.cube_map,
    );

    if !gl_state.skybox.textures_loaded {
        for (img, bind_pt) in map_state.skybox.img_iter() {
            let format = if img.has_alpha() {
                webgl::ColorFormat::RGBA
            } else {
                webgl::ColorFormat::RGB
            };
            if img.has_eight_bit_channels() {
                webgl::tex_image_2d_u8(
                    bind_pt,
                    0,
                    format,
                    img.get_width() as i32,
                    img.get_height() as i32,
                    format,
                    img.get_data(),
                );
            } else {
                unimplemented!();
            }
        }

        gl_state.skybox.textures_loaded = true;
    }

    webgl::draw_arrays(webgl::RenderingPrimitive::Triangles, 0, 3 * 2 * 6);
    webgl::depth_func(webgl::DepthFunc::Less);

    Ok(())
}
