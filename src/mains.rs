use geometry;
use na;
use wasm_bindgen::prelude::*;
use webgl;

const VERTEX_SHADER_SRC: &str = r#"#version 300 es

in vec4 a_position;
in vec3 a_normal;

uniform mat4 u_worldViewProjection;
uniform mat4 u_world;

out vec3 v_normal;

void main() {
    gl_Position = u_worldViewProjection * a_position;

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

    let world_view_proj_uni_loc =
        webgl::get_uniform_location(&program, "u_worldViewProjection").expect(
            "There is no uniform with the name \"u_worldViewProjection\"",
        );
    let world_uni_loc = webgl::get_uniform_location(&program, "u_world")
        .expect("There is no uniform with the name \"u_world\"");
    let color_uni_loc = webgl::get_uniform_location(&program, "u_color")
        .expect("There is no uniform with the name \"u_color\"");
    let reverse_light_dir_uni_loc = webgl::get_uniform_location(
        &program,
        "u_reverseLightDirection",
    ).expect(
        "There is no uniform with the name \"u_reverseLightDirection\"",
    );

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
        &[
            // Hexagon
            0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////
            0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////
            0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////
            0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////
            0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////
            0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////
            0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////
            0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////
            0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////
            0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////
            0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////
            0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////
            0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////
            0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////
            0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////
            0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////
            0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////
            0.0, 0.0, 1.0, ////////////////////////////////////////////////////////////////
            // Prism: first face, northeast
            geometry::SQRT_3_ON_2, 0.5, 0.0, ////////////////////////////////////////////////////////////////
            geometry::SQRT_3_ON_2, 0.5, 0.0, ////////////////////////////////////////////////////////////////
            geometry::SQRT_3_ON_2, 0.5, 0.0, ////////////////////////////////////////////////////////////////
            geometry::SQRT_3_ON_2, 0.5, 0.0, ////////////////////////////////////////////////////////////////
            geometry::SQRT_3_ON_2, 0.5, 0.0, ////////////////////////////////////////////////////////////////
            geometry::SQRT_3_ON_2, 0.5, 0.0, ////////////////////////////////////////////////////////////////
            // Prism: second face, north
            0.0, 1.0, 0.0, ////////////////////////////////////////////////////////////////
            0.0, 1.0, 0.0, ////////////////////////////////////////////////////////////////
            0.0, 1.0, 0.0, ////////////////////////////////////////////////////////////////
            0.0, 1.0, 0.0, ////////////////////////////////////////////////////////////////
            0.0, 1.0, 0.0, ////////////////////////////////////////////////////////////////
            0.0, 1.0, 0.0, ////////////////////////////////////////////////////////////////
            // Prism: third face, northwest
            -geometry::SQRT_3_ON_2, 0.5, 0.0, ////////////////////////////////////////////////////////////////
            -geometry::SQRT_3_ON_2, 0.5, 0.0, ////////////////////////////////////////////////////////////////
            -geometry::SQRT_3_ON_2, 0.5, 0.0, ////////////////////////////////////////////////////////////////
            -geometry::SQRT_3_ON_2, 0.5, 0.0, ////////////////////////////////////////////////////////////////
            -geometry::SQRT_3_ON_2, 0.5, 0.0, ////////////////////////////////////////////////////////////////
            -geometry::SQRT_3_ON_2, 0.5, 0.0, ////////////////////////////////////////////////////////////////
            // Prism: fourth face, southwest
            -geometry::SQRT_3_ON_2, -0.5, 0.0, ////////////////////////////////////////////////////////////////
            -geometry::SQRT_3_ON_2, -0.5, 0.0, ////////////////////////////////////////////////////////////////
            -geometry::SQRT_3_ON_2, -0.5, 0.0, ////////////////////////////////////////////////////////////////
            -geometry::SQRT_3_ON_2, -0.5, 0.0, ////////////////////////////////////////////////////////////////
            -geometry::SQRT_3_ON_2, -0.5, 0.0, ////////////////////////////////////////////////////////////////
            -geometry::SQRT_3_ON_2, -0.5, 0.0, ////////////////////////////////////////////////////////////////
            // Prism: fifth face, south
            0.0, -1.0, 0.0, ////////////////////////////////////////////////////////////////
            0.0, -1.0, 0.0, ////////////////////////////////////////////////////////////////
            0.0, -1.0, 0.0, ////////////////////////////////////////////////////////////////
            0.0, -1.0, 0.0, ////////////////////////////////////////////////////////////////
            0.0, -1.0, 0.0, ////////////////////////////////////////////////////////////////
            0.0, -1.0, 0.0, ////////////////////////////////////////////////////////////////
            // Prism: sixth face, southeast
            geometry::SQRT_3_ON_2, -0.5, 0.0, ////////////////////////////////////////////////////////////////
            geometry::SQRT_3_ON_2, -0.5, 0.0, ////////////////////////////////////////////////////////////////
            geometry::SQRT_3_ON_2, -0.5, 0.0, ////////////////////////////////////////////////////////////////
            geometry::SQRT_3_ON_2, -0.5, 0.0, ////////////////////////////////////////////////////////////////
            geometry::SQRT_3_ON_2, -0.5, 0.0, ////////////////////////////////////////////////////////////////
            geometry::SQRT_3_ON_2, -0.5, 0.0, ////////////////////////////////////////////////////////////////
        ],
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

    // Compute a transformation matrix:
    // scale, project, translate, rotate
    let mut view = na::Matrix4::new_perspective(
        webgl::get_canvas_width() / webgl::get_canvas_height(),
        0.875,
        1.0,
        2000.0,
    );
    view *= na::Matrix4::new_translation(&na::Vector3::new(0.0, 0.0, -400.0));
    let world =
        na::Matrix4::new_rotation(na::Vector3::new(rot_x, rot_y, rot_z));
    view *= world;
    view *= na::Matrix4::new_scaling(40.0);

    // Pass in the world matrix
    webgl::uniform_matrix4fv(&world_uni_loc, world.as_slice());

    // Pass in the world view/projection matrix
    webgl::uniform_matrix4fv(&world_view_proj_uni_loc, view.as_slice());

    // Set the color
    webgl::uniform4fv(&color_uni_loc, &[0.1725, 0.6902, 0.2157, 1.0]);

    // Set the light direction
    webgl::uniform3fv(
        &reverse_light_dir_uni_loc,
        na::Vector3::new(0.5, 0.7, 1.0).normalize().as_slice(),
    );
    ////////////////////////////////////////////////////////////////////////

    // Draw!
    webgl::draw_arrays(
        webgl::RenderingPrimitive::Triangles,
        0,
        geometry::HEXAGONAL_PRISM.len() as i32 / 3,
    );
    ////////////////////////////////////////////////////////////////////////
}
