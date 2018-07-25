use geometry;
use na;
use wasm_bindgen::prelude::*;
use webgl;

const VERTEX_SHADER_SRC: &str = r#"#version 300 es

in vec4 a_position;

uniform mat4 u_matrix;

void main() {
    gl_Position = u_matrix * a_position;
}
"#;

const FRAGMENT_SHADER_SRC: &str = r#"#version 300 es

precision mediump float;

out vec4 outColor;

void main() {
    // Just set the output to a constant pink color.
    outColor = vec4(0.9, 0.2, 0.45, 1);
}
"#;

#[wasm_bindgen]
pub fn test0() {
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
    let position_buffer = webgl::create_buffer();
    webgl::bind_buffer(webgl::BufferType::ArrayBuffer, &position_buffer);

    webgl::buffer_data_f32(
        webgl::BufferType::ArrayBuffer,
        geometry::HEXAGON_TRI_FAN,
        //.iter()
        //.map(|x| x * 80.0)
        //.collect::<Vec<_>>()
        //.as_slice(),
        webgl::UsageType::StaticDraw,
        0,
        None,
    );

    let vao = webgl::create_vertex_array();
    webgl::bind_vertex_array(&vao);
    webgl::enable_vertex_attr_array(position_attr_loc);
    webgl::vertex_attr_ptr(
        position_attr_loc,
        2,                      // Two components per iteration
        webgl::DataType::Float, // The data is `f32`s
        false,                  // Don't normalize to clip space
        0,                      // Stride (in bytes)
        0,                      // Offset (in bytes)
    );

    webgl::resize_canvas_to_display();
    webgl::reset_viewport();

    // Clear the canvas
    webgl::clear_color(0.0, 0.0, 0.0, 0.0);
    webgl::clear(webgl::COLOR_BUFFER_BIT);

    // Tell it to use our program (pair of shaders)
    webgl::use_program(&program);

    // Compute a transformation matrix:
    // scale, project, translate, rotate
    let mut m = geometry::new_projection(
        webgl::get_canvas_width() as f32,
        webgl::get_canvas_height() as f32,
        400.0,
    );
    m *= na::Matrix4::new_translation(&na::Vector3::new(200.0, 200.0, 0.0));
    m *= na::Matrix4::new_rotation(na::Vector3::new(0.0, 0.0, 0.8));
    m *= na::Matrix4::new_scaling(40.0);

    // Pass in the transformation matrix
    webgl::uniform_matrix4fv(&matrix_uni_loc, m.as_slice());

    // Bind the attribute/buffer set we want
    webgl::bind_vertex_array(&vao);

    // Draw!
    webgl::draw_arrays(
        webgl::RenderingPrimitive::TriangleFan,
        0,
        (geometry::HEXAGON_TRI_FAN.len() / 2) as i32,
    );
}
