use geometry;
use na;
use random;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use webgl;

struct GlState {
    world_uni_loc:             webgl::WebGLUniformLocation,
    world_view_proj_uni_loc:   webgl::WebGLUniformLocation,
    reverse_light_dir_uni_loc: webgl::WebGLUniformLocation,
    color_uni_loc:             webgl::WebGLUniformLocation,
    displacement_uni_loc:      webgl::WebGLUniformLocation,
}

struct MapState {
    terrain_colors: Vec<[f32; 4]>,
}

lazy_static! {
    static ref GL_STATE: Mutex<Option<GlState>> = Mutex::new(None);
    static ref MAP_STATE: Mutex<MapState> = Mutex::new(MapState::new());
}

const VERTEX_SHADER_SRC: &str = r#"#version 300 es

in vec4 a_position;
in vec3 a_normal;

uniform mat4 u_worldViewProjection;
uniform mat4 u_world;
uniform vec3 u_displacement;

out vec3 v_normal;

void main() {
    vec4 displaced = a_position + vec4(u_displacement, 0);
    gl_Position = u_worldViewProjection * displaced;

    v_normal = mat3(u_world) * a_normal;
}
"#;

const FRAGMENT_SHADER_SRC: &str = r#"#version 300 es

precision mediump float;

in vec3 v_normal;

uniform vec3 u_reverseLightDirection;
uniform vec4 u_color;

out vec4 outColor;

void main() {
    vec3 normal = normalize(v_normal);

    float light = dot(normal, u_reverseLightDirection);

    outColor = u_color;

    outColor.rgb *= light;
}
"#;

impl MapState {
    #[inline]
    pub fn new() -> Self {
        MapState {
            terrain_colors: Vec::new(),
        }
    }
}

#[wasm_bindgen]
pub fn init() {
    let vertex_shader = webgl::create_shader(
        webgl::ShaderType::VertexShader,
        VERTEX_SHADER_SRC,
    ).expect("Failed to create vertex shader");
    let fragment_shader = webgl::create_shader(
        webgl::ShaderType::FragmentShader,
        FRAGMENT_SHADER_SRC,
    ).expect("Failed to create fragment shader");

    let program = webgl::create_program(&vertex_shader, &fragment_shader)
        .expect("Failed to link GLSL program");

    let world_view_proj_uni_loc = webgl::get_uniform_location(
        &program,
        "u_worldViewProjection",
    ).expect("There is no uniform with the name \"u_worldViewProjection\"");
    let world_uni_loc = webgl::get_uniform_location(&program, "u_world")
        .expect("There is no uniform with the name \"u_world\"");
    let color_uni_loc = webgl::get_uniform_location(&program, "u_color")
        .expect("There is no uniform with the name \"u_color\"");
    let displacement_uni_loc =
        webgl::get_uniform_location(&program, "u_displacement")
            .expect("There is no uniform with the name \"u_displacement\"");
    let reverse_light_dir_uni_loc =
        webgl::get_uniform_location(&program, "u_reverseLightDirection")
            .expect(
                "There is no uniform with the name \
                 \"u_reverseLightDirection\"",
            );

    match GL_STATE.lock() {
        Ok(mut gl_state) =>
            *gl_state = Some(GlState {
                world_uni_loc,
                world_view_proj_uni_loc,
                reverse_light_dir_uni_loc,
                color_uni_loc,
                displacement_uni_loc,
            }),
        _ => unreachable!(),
    }
    ////////////////////////////////////////////////////////////////////////

    let position_attr_loc = webgl::get_attr_location(&program, "a_position");
    if position_attr_loc < 0 {
        panic!("There is no attribute with the name \"a_position\"");
    }
    let position_attr_loc = position_attr_loc as u32;
    let normal_attr_loc = webgl::get_attr_location(&program, "a_normal");
    if normal_attr_loc < 0 {
        panic!("There is no attribute with the name \"a_normal\"");
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

    let prism_vao = webgl::create_vertex_array();
    webgl::bind_vertex_array(&prism_vao);
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

    webgl::resize_canvas_to_display();
    webgl::reset_viewport();

    // Clear the canvas
    webgl::clear_color(0.0, 0.0, 0.0, 0.0);
    webgl::clear(webgl::COLOR_BUFFER_BIT | webgl::DEPTH_BUFFER_BIT);

    // Capabilities
    webgl::enable(webgl::Capability::CullFace);
    webgl::enable(webgl::Capability::DepthTest);
    //////////////////////////////////////////////////

    // Tell it to use our program (pair of shaders)
    webgl::use_program(&program);
    ////////////////////////////////////////////////////////////////////////

    // Bind the attribute/buffer set we want
    webgl::bind_vertex_array(&prism_vao);
    ////////////////////////////////////////////////////////////////////////

    // Come up with some random colors
    match MAP_STATE.lock() {
        Ok(mut map_state) =>
            map_state.terrain_colors = (0..geometry::TERRAIN_TRANSLATIONS
                .len())
                .map(|_| {
                    [
                        random::next_f32(),
                        random::next_f32(),
                        random::next_f32(),
                        1.0,
                    ]
                }).collect(),
        _ => unreachable!(),
    }
}

#[wasm_bindgen]
pub fn render(
    trans_x: f32,
    trans_y: f32,
    trans_z: f32,
    lookat_eye_x: f32,
    lookat_eye_y: f32,
    lookat_eye_z: f32,
    rot_x: f32,
    rot_y: f32,
    rot_z: f32,
) {
    // Retrieve GL state
    let gl_state_lock = match GL_STATE.lock() {
        Ok(gl_s) => gl_s,
        _ => unreachable!(),
    };
    let gl_state = gl_state_lock.as_ref().unwrap_or_else(|| {
        panic!("Did not call `init` before callng `render`")
    });

    // Retreieve map state
    let map_state = match MAP_STATE.lock() {
        Ok(map_s) => map_s,
        _ => unreachable!(),
    };

    // Reset the canvas
    webgl::resize_canvas_to_display();
    webgl::reset_viewport();
    webgl::clear_color(0.0, 0.0, 0.0, 0.0);
    webgl::clear(webgl::COLOR_BUFFER_BIT | webgl::DEPTH_BUFFER_BIT);

    // Compute transformation matrices
    let mut view_proj = na::Matrix4::new_perspective(
        webgl::get_canvas_width() / webgl::get_canvas_height(),
        0.875,
        1.0,
        2000.0,
    ) * na::Matrix4::new_translation(&na::Vector3::new(
        trans_x, trans_y, trans_z,
    ));
    let world =
        na::Matrix4::new_rotation(na::Vector3::new(rot_x, rot_y, rot_z)); // * na::Matrix4::new_scaling(1.0);
    let mut view = na::Matrix4::look_at_rh(
        &na::Point3::new(lookat_eye_x, lookat_eye_y, lookat_eye_z),
        &na::Point3::origin(),
        &na::Vector3::y(),
    );
    if !view.try_inverse_mut() {
        ::js::log("`view` invert failure");
    }
    view_proj *= view;
    view_proj *= world;

    // Pass in the world matrix
    webgl::uniform_matrix4fv(&gl_state.world_uni_loc, world.as_slice());

    // Pass in the world view/projection matrix
    webgl::uniform_matrix4fv(
        &gl_state.world_view_proj_uni_loc,
        view_proj.as_slice(),
    );

    //let green = [0.1725f32, 0.6902, 0.2157, 1.0];

    // Set the light direction
    webgl::uniform3fv(
        &gl_state.reverse_light_dir_uni_loc,
        na::Vector3::new(0.5, 0.7, 1.0).normalize().as_slice(),
    );

    for (i, trans) in geometry::TERRAIN_TRANSLATIONS.iter().enumerate() {
        // Set the color
        webgl::uniform4fv(
            &gl_state.color_uni_loc,
            &map_state.terrain_colors[i],
        );

        // Set the displacement
        webgl::uniform3fv(&gl_state.displacement_uni_loc, trans);
        ////////////////////////////////////////////////////////////////////

        // Draw!
        webgl::draw_arrays(
            webgl::RenderingPrimitive::Triangles,
            0,
            geometry::HEXAGONAL_PRISM.len() as i32 / 3,
        );
        ////////////////////////////////////////////////////////////////////
    }
}
