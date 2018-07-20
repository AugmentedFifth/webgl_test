#![feature(use_extern_macros, wasm_custom_section, wasm_import_module)]

mod js;
mod webgl;

pub use js::*;
pub use webgl::*;

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

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
pub fn test0(name: &str) {
    js::log(&format!("Hello, {}!", name));
}
