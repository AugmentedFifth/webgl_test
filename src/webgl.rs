use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// https://developer.mozilla.org/en-US/docs/Web/API/WebGLShader
    pub type WebGLShader;
}

#[wasm_bindgen(module = "./index")]
extern "C" {
    fn create_shader_sys(
        shader_type: u32,
        source: &str,
    ) -> Option<WebGLShader>;
}

#[repr(u32)]
pub enum ShaderType {
    FragmentShader = 0x8B30,
    VertexShader = 0x8B31,
}

/// Create a shader with the specified type using GLSL source code.
pub fn create_shader(
    shader_type: ShaderType,
    source: &str,
) -> Option<WebGLShader> {
    create_shader_sys(shader_type as u32, source)
}
