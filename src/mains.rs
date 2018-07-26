use geometry;
use na;
use wasm_bindgen::prelude::*;
use webgl;

const VERTEX_SHADER_SRC: &str = r#"#version 300 es

in vec4 a_position;
in vec4 a_color;

uniform mat4 u_matrix;

out vec4 v_color;

void main() {
    gl_Position = u_matrix * a_position;

    v_color = a_color;
}
"#;

const FRAGMENT_SHADER_SRC: &str = r#"#version 300 es

precision mediump float;

in vec4 v_color;

out vec4 outColor;

void main() {
    outColor = v_color;
}
"#;

#[wasm_bindgen]
pub fn test0(rot_x: f32, rot_y: f32, rot_z: f32) {
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

    let matrix_uni_loc = webgl::get_uniform_location(&program, "u_matrix")
        .expect("There is no uniform with the name \"u_matrix\"");

    let position_attr_loc = webgl::get_attr_location(&program, "a_position");
    if position_attr_loc < 0 {
        panic!("There is no attribute with the name \"a_position\"");
    }
    let position_attr_loc = position_attr_loc as u32;

    let color_attr_loc = webgl::get_attr_location(&program, "a_color");
    if color_attr_loc < 0 {
        panic!("There is no attribute with the name \"a_color\"");
    }
    let color_attr_loc = color_attr_loc as u32;
    //////////////////////////////////////////////////////////////////////////

    let hex_buffer = webgl::create_buffer();
    webgl::bind_buffer(webgl::BufferType::ArrayBuffer, &hex_buffer);
    webgl::buffer_data_f32(
        webgl::BufferType::ArrayBuffer,
        geometry::HEXAGON_TRI_FAN,
        webgl::UsageType::StaticDraw,
        0,
        None,
    );

    let hex_color_buffer = webgl::create_buffer();
    webgl::bind_buffer(webgl::BufferType::ArrayBuffer, &hex_color_buffer);
    webgl::buffer_data(
        webgl::BufferType::ArrayBuffer,
        &[
            200, 40, 80, //////////////
            200, 40, 80, //////////////
            200, 40, 80, //////////////
            10, 50, 110, //////////////
            10, 50, 110, //////////////
            100, 190, 50, //////////////
            100, 190, 50, //////////////
            50, 100, 190, //////////////
        ],
        webgl::UsageType::StaticDraw,
        0,
        None,
    );

    let prism_buffer = webgl::create_buffer();
    webgl::bind_buffer(webgl::BufferType::ArrayBuffer, &prism_buffer);
    webgl::buffer_data_f32(
        webgl::BufferType::ArrayBuffer,
        geometry::HEXAGONAL_PRISM_STRIP,
        webgl::UsageType::StaticDraw,
        0,
        None,
    );

    let prism_color_buffer = webgl::create_buffer();
    webgl::bind_buffer(webgl::BufferType::ArrayBuffer, &prism_color_buffer);
    webgl::buffer_data(
        webgl::BufferType::ArrayBuffer,
        &[
            164, 91, 133, //////////////
            164, 91, 133, //////////////
            164, 91, 133, //////////////
            164, 91, 133, //////////////
            76, 155, 34, //////////////
            76, 155, 34, //////////////
            156, 196, 138, //////////////
            156, 196, 138, //////////////
            63, 199, 157, //////////////
            63, 199, 157, //////////////
            58, 46, 192, //////////////
            58, 46, 192, //////////////
            48, 171, 226, //////////////
            48, 171, 226, //////////////
        ],
        webgl::UsageType::StaticDraw,
        0,
        None,
    );
    //////////////////////////////////////////////////////////////////////////

    let hex_vao = webgl::create_vertex_array();
    webgl::bind_vertex_array(&hex_vao);
    webgl::bind_buffer(webgl::BufferType::ArrayBuffer, &hex_buffer);
    webgl::enable_vertex_attr_array(position_attr_loc);
    webgl::vertex_attr_ptr(
        position_attr_loc,
        2,                      // Two components per iteration
        webgl::DataType::Float, // The data is `f32`s
        false,                  // Don't normalize to clip space
        0,                      // Stride (in bytes)
        0,                      // Offset (in bytes)
    );

    webgl::bind_buffer(webgl::BufferType::ArrayBuffer, &hex_color_buffer);
    webgl::enable_vertex_attr_array(color_attr_loc);
    webgl::vertex_attr_ptr(
        color_attr_loc,
        3,                             // Three components per iteration
        webgl::DataType::UnsignedByte, // The data is `u8`s
        true,                          // Normalize to [0.0, 1.0]
        0,                             // Stride (in bytes)
        0,                             // Offset (in bytes)
    );

    let prism_vao = webgl::create_vertex_array();
    webgl::bind_vertex_array(&prism_vao);
    webgl::bind_buffer(webgl::BufferType::ArrayBuffer, &prism_buffer);
    webgl::enable_vertex_attr_array(position_attr_loc);
    webgl::vertex_attr_ptr(
        position_attr_loc,
        3,                      // Three components per iteration
        webgl::DataType::Float, // The data is `f32`s
        false,                  // Don't normalize to clip space
        0,                      // Stride (in bytes)
        0,                      // Offset (in bytes)
    );

    webgl::bind_buffer(webgl::BufferType::ArrayBuffer, &prism_color_buffer);
    webgl::enable_vertex_attr_array(color_attr_loc);
    webgl::vertex_attr_ptr(
        color_attr_loc,
        3,                             // Three components per iteration
        webgl::DataType::UnsignedByte, // The data is `u8`s
        true,                          // Normalize to [0.0, 1.0]
        0,                             // Stride (in bytes)
        0,                             // Offset (in bytes)
    );
    //////////////////////////////////////////////////////////////////////////

    webgl::resize_canvas_to_display();
    webgl::reset_viewport();

    // Clear the canvas
    webgl::clear_color(0.0, 0.0, 0.0, 0.0);
    webgl::clear(webgl::COLOR_BUFFER_BIT | webgl::DEPTH_BUFFER_BIT);

    // Capabilities
    webgl::enable(webgl::Capability::CullFace);
    webgl::enable(webgl::Capability::DepthTest);
    //////////////////////////////////////////////////////////////////////////

    // Tell it to use our program (pair of shaders)
    webgl::use_program(&program);
    //////////////////////////////////////////////////////////////////////////

    // Bind the attribute/buffer set we want
    webgl::bind_vertex_array(&hex_vao);
    //////////////////////////////////////////////////////////////////////////

    // Compute a transformation matrix:
    // scale, project, translate, rotate
    let mut m = geometry::new_projection(
        webgl::get_canvas_width() as f32,
        webgl::get_canvas_height() as f32,
        400.0,
    );
    m *= na::Matrix4::new_translation(&na::Vector3::new(200.0, 200.0, 0.0));
    m *= na::Matrix4::new_rotation(na::Vector3::new(rot_x, rot_y, rot_z));
    m *= na::Matrix4::new_scaling(40.0);

    // Pass in the transformation matrix
    webgl::uniform_matrix4fv(&matrix_uni_loc, m.as_slice());
    //////////////////////////////////////////////////////////////////////////

    // Draw!
    webgl::draw_arrays(
        webgl::RenderingPrimitive::TriangleFan,
        0,
        (geometry::HEXAGON_TRI_FAN.len() / 2) as i32,
    );
    //////////////////////////////////////////////////////////////////////////

    // Bind the attribute/buffer set we want
    webgl::bind_vertex_array(&prism_vao);
    //////////////////////////////////////////////////////////////////////////

    // Pass in the transformation matrix
    webgl::uniform_matrix4fv(&matrix_uni_loc, m.as_slice());
    //////////////////////////////////////////////////////////////////////////

    // Draw!
    webgl::draw_arrays(
        webgl::RenderingPrimitive::TriangleStrip,
        0,
        (geometry::HEXAGONAL_PRISM_STRIP.len() / 3) as i32,
    );
}
