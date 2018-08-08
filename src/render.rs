use geometry;
use mains;
use map;
use na;
use physics;
use std::{f32::consts::FRAC_PI_2, sync::Mutex};
use webgl;

struct GlState {
    world_uni_loc:             webgl::WebGLUniformLocation,
    world_view_proj_uni_loc:   webgl::WebGLUniformLocation,
    reverse_light_dir_uni_loc: webgl::WebGLUniformLocation,
    color_uni_loc:             webgl::WebGLUniformLocation,
    displacement_uni_loc:      webgl::WebGLUniformLocation,
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
    // vec3 normal = normalize(v_normal);

    float light = dot(v_normal, u_reverseLightDirection);

    outColor = vec4(u_color * light, 1.0);
}
"#;

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

    *GL_STATE.lock().unwrap() = Some(GlState {
        world_uni_loc,
        world_view_proj_uni_loc,
        reverse_light_dir_uni_loc,
        color_uni_loc,
        displacement_uni_loc,
    });
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
}

pub fn render() {
    // Retrieve GL state
    let gl_state_lock = GL_STATE.lock().unwrap();
    let gl_state = gl_state_lock.as_ref().unwrap_or_else(|| {
        panic!("Did not call `init` before callng `render`")
    });

    // Retrieve physical state
    let world = physics::WORLD.lock().unwrap();
    let player_body =
        world.rigid_body(*physics::PLAYER.lock().unwrap()).unwrap();

    // Retrieve player-specific state
    let player_state = mains::PLAYER_STATE.lock().unwrap();

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
    );
    let world =
        na::Matrix4::new_rotation(na::Vector3::new(-FRAC_PI_2, 0.0, 0.0));
    let player_com = player_body.center_of_mass();
    let view = na::Matrix4::look_at_rh(
        &player_com,
        &(player_com + player_state.orient.unwrap()),
        &na::Vector3::y(),
    );
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
        na::Vector3::new(0.7, 1.0, 0.5).normalize().as_slice(),
    );

    for (hex, (x, y)) in map::MAP.lock().unwrap().iter() {
        // Set the color
        webgl::uniform3fv(
            &gl_state.color_uni_loc,
            hex.color.as_floating().rgb(),
        );

        // Set the coordinates
        webgl::uniform3f(&gl_state.displacement_uni_loc, *x, *y, hex.height);
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
