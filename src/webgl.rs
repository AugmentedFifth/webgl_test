use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// https://developer.mozilla.org/en-US/docs/Web/API/WebGLShader
    pub type WebGLShader;

    /// https://developer.mozilla.org/en-US/docs/Web/API/WebGLProgram
    pub type WebGLProgram;

    /// https://developer.mozilla.org/en-US/docs/Web/API/WebGLUniformLocation
    pub type WebGLUniformLocation;

    /// https://developer.mozilla.org/en-US/docs/Web/API/WebGLBuffer
    pub type WebGLBuffer;
}

#[wasm_bindgen(module = "./index")]
extern "C" {
    fn create_shader_sys(
        shader_type: u32,
        source: &str,
    ) -> Option<WebGLShader>;

    /// Links a program from a vertex shader and a fragment shader.
    pub fn create_program(
        vertex_shader: &WebGLShader,
        fragment_shader: &WebGLShader,
    ) -> Option<WebGLProgram>;

    /// Get the location of a uniform variable.
    pub fn get_uniform_location(
        prog: &WebGLProgram,
        name: &str,
    ) -> Option<WebGLUniformLocation>;

    /// Get the location of an attribute. Returns `-1` if there is no such
    /// attribute.
    pub fn get_attr_location(prog: &WebGLProgram, name: &str) -> i32;

    /// Creates and initializes a `WebGLBuffer` for storing data such as
    /// vertices or colors.
    pub fn create_buffer() -> WebGLBuffer;

    fn bind_buffer_sys(target: u32, buffer: &WebGLBuffer);
}

#[repr(u32)]
pub enum ShaderType {
    FragmentShader = 0x8B30,
    VertexShader = 0x8B31,
}

#[repr(u32)]
pub enum BufferType {
    ArrayBuffer = 0x8892,
    ElementArrayBuffer = 0x8893,
    PixelPackBuffer = 0x88EB,
    PixelUnpackBuffer = 0x88EC,
    UniformBuffer = 0x8A11,
    TransformFeedbackBuffer = 0x8C8E,
    CopyReadBuffer = 0x8F36,
    CopyWriteBuffer = 0x8F37,
}

/// Create a shader with the specified type using GLSL source code.
pub fn create_shader(
    shader_type: ShaderType,
    source: &str,
) -> Option<WebGLShader> {
    create_shader_sys(shader_type as u32, source)
}

/// Binds a `WebGLBuffer` to a target buffer type.
pub fn bind_buffer(target: BufferType, buffer: &WebGLBuffer) {
    bind_buffer_sys(target as u32, buffer);
}
