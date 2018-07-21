use wasm_bindgen::prelude::*;
use webgl;

const VERTEX_SHADER_SRC: &str = r#"#version 300 es

in vec2 a_position;

uniform vec2 u_resolution;

void main() {
    // convert the position from pixels to 0.0 to 1.0
    vec2 zeroToOne = a_position / u_resolution;

    // convert from 0->1 to 0->2
    vec2 zeroToTwo = zeroToOne * 2.0;

    // convert from 0->2 to -1->+1 (clipspace)
    vec2 clipSpace = zeroToTwo - 1.0;

    gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);
}
"#;

const FRAGMENT_SHADER_SRC: &str = r#"#version 300 es

// fragment shaders don't have a default precision so we need
// to pick one. mediump is a good default. It means "medium precision"
precision mediump float;

// we need to declare an output for the fragment shader
out vec4 outColor;

void main() {
    // Just set the output to a constant reddish-purple
    outColor = vec4(1, 0, 0.5, 1);
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

    let resolution_uni_loc =
        webgl::get_uniform_location(&program, "u_resolution")
            .expect("There is no uniform with the name `u_resolution`");

    let position_attr_loc = webgl::get_attr_location(&program, "a_position");
    if position_attr_loc < 0 {
        panic!("There is no attribute with the name `a_position`");
    }
    let position_attr_loc = position_attr_loc as u32;
    let position_buffer = webgl::create_buffer();
    webgl::bind_buffer(webgl::BufferType::ArrayBuffer, &position_buffer);

    const POSITIONS: &[f32] = &[
        10.0, 20.0, // (10, 20)
        80.0, 20.0, // (80, 20)
        10.0, 30.0, // (10, 30)
        10.0, 30.0, // (10, 30)
        80.0, 20.0, // (80, 20)
        80.0, 30.0, // (80, 30)
    ];
    webgl::buffer_data_f32(
        webgl::BufferType::ArrayBuffer,
        POSITIONS,
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
    webgl::clear_color(0.5, 0.0, 0.5, 0.0);
    webgl::clear(webgl::COLOR_BUFFER_BIT);

    // Tell it to use our program (pair of shaders)
    webgl::use_program(&program);

    // Pass in the canvas resolution so we can convert from pixels to
    // clipspace in the shader
    webgl::uniform2f(
        &resolution_uni_loc,
        webgl::get_canvas_width() as f32,
        webgl::get_canvas_height() as f32,
    );

    // Bind the attribute/buffer set we want
    webgl::bind_vertex_array(&vao);

    // Draw!
    webgl::draw_arrays(
        webgl::RenderingPrimitive::Triangles,
        0,
        (POSITIONS.len() / 2) as i32,
    );
}
