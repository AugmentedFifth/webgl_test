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

    /// https://developer.mozilla.org/en-US/docs/Web/API/WebGLVertexArrayObject
    pub type WebGLVertexArrayObject;
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

    fn buffer_data_sys(
        target: u32,
        src_data: &[u8],
        usage: u32,
        src_offset: u32,
        length: u32,
    );
    fn buffer_data_u16_sys(
        target: u32,
        src_data: &[u16],
        usage: u32,
        src_offset: u32,
        length: u32,
    );
    fn buffer_data_f32_sys(
        target: u32,
        src_data: &[f32],
        usage: u32,
        src_offset: u32,
        length: u32,
    );

    /// WebGL 2 function that creates a vertex array object (VAO) pointing to
    /// vertex array data and which provides names for different sets of vertex
    /// data.
    pub fn create_vertex_array() -> WebGLVertexArrayObject;

    /// WebGL 2 function that binds a vertex array object (VAO) to the current
    /// buffer.
    pub fn bind_vertex_array(vao: &WebGLVertexArrayObject);

    /// Enables the vertex attribute at the specified index.
    pub fn enable_vertex_attr_array(index: u32);

    fn vertex_attr_ptr_sys(
        index: u32,
        size: i32,
        data_type: u32,
        normalized: bool,
        stride: i32,
        offset: i32,
    );

    pub fn get_canvas_width() -> f32;

    pub fn get_canvas_height() -> f32;

    pub fn resize_canvas_to_display();

    pub fn reset_viewport();

    /// Sets the color to be used when calling `clear(u32)`.
    pub fn clear_color(r: f32, g: f32, b: f32, a: f32);

    /// Clears the specified buffers, the specification being the bitwise OR of
    /// one or more of:
    ///
    /// * `DEPTH_BUFFER_BIT`
    /// * `STENCIL_BUFFER_BIT`
    /// * `COLOR_BUFFER_BIT`
    pub fn clear(mask: u32);

    /// Sets the specified `WebGLProgram` as part of the current rendering
    /// state.
    pub fn use_program(prog: &WebGLProgram);

    fn draw_arrays_sys(mode: u32, first: i32, count: i32);

    fn draw_elements_sys(mode: u32, count: i32, data_type: u32, offset: i32);

    /// Specifies the value of a uniform.
    pub fn uniform2ui(loc: &WebGLUniformLocation, x: u32, y: u32);

    /// Specifies the value of a uniform.
    pub fn uniform2f(loc: &WebGLUniformLocation, x: f32, y: f32);

    /// Specifies the value of a uniform.
    pub fn uniform3f(loc: &WebGLUniformLocation, x: f32, y: f32, z: f32);

    /// Specifies the value of a uniform.
    pub fn uniform_matrix4fv(loc: &WebGLUniformLocation, data: &[f32]);

    fn enable_sys(cap: u32);
}

pub const DEPTH_BUFFER_BIT: u32 = 0x0000_0100;
pub const STENCIL_BUFFER_BIT: u32 = 0x0000_0400;
pub const COLOR_BUFFER_BIT: u32 = 0x0000_4000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ShaderType {
    FragmentShader = 0x8B30,
    VertexShader = 0x8B31,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum UsageType {
    StreamDraw = 0x88E0,
    StreamRead = 0x88E1,
    StreamCopy = 0x88E2,
    StaticDraw = 0x88E4,
    StaticRead = 0x88E5,
    StaticCopy = 0x88E6,
    DynamicDraw = 0x88E8,
    DynamicRead = 0x88E9,
    DynamicCopy = 0x88EA,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum DataType {
    Byte = 0x1400,
    UnsignedByte = 0x1401,
    Short = 0x1402,
    UnsignedShort = 0x1403,
    Int = 0x1404,
    UnsignedInt = 0x1405,
    Float = 0x1406,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum ElementDataType {
    UnsignedByte = 0x1401,
    UnsignedShort = 0x1403,
    UnsignedInt = 0x1405,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum RenderingPrimitive {
    Points = 0x0000,
    Lines = 0x0001,
    LineLoop = 0x0002,
    LineStrip = 0x0003,
    Triangles = 0x0004,
    TriangleStrip = 0x0005,
    TriangleFan = 0x0006,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Capability {
    CullFace = 0x0B44,
    DepthTest = 0x0B71,
    StencilTest = 0x0B90,
    Dither = 0x0BD0,
    Blend = 0x0BE2,
    ScissorTest = 0x0C11,
    PolygonOffsetFill = 0x8037,
    SampleAlphaToCoverage = 0x809E,
    SampleCoverage = 0x80A0,
    RasterizerDiscard = 0x8C89,
}

/// Create a shader with the specified type using GLSL source code.
#[inline]
pub fn create_shader(
    shader_type: ShaderType,
    source: &str,
) -> Option<WebGLShader> {
    create_shader_sys(shader_type as u32, source)
}

/// Binds a `WebGLBuffer` to a target buffer type.
#[inline]
pub fn bind_buffer(target: BufferType, buffer: &WebGLBuffer) {
    bind_buffer_sys(target as u32, buffer);
}

/// Initializes the specified buffer object's data store.
#[inline]
pub fn buffer_data(
    target: BufferType,
    src_data: &[u8],
    usage: UsageType,
    src_offset: u32,
    length: Option<u32>,
) {
    buffer_data_sys(
        target as u32,
        src_data,
        usage as u32,
        src_offset,
        length.unwrap_or(0),
    );
}

/// Initializes the specified buffer object's data store.
#[inline]
pub fn buffer_data_u16(
    target: BufferType,
    src_data: &[u16],
    usage: UsageType,
    src_offset: u32,
    length: Option<u32>,
) {
    buffer_data_u16_sys(
        target as u32,
        src_data,
        usage as u32,
        src_offset,
        length.unwrap_or(0),
    );
}

/// Initializes the specified buffer object's data store.
#[inline]
pub fn buffer_data_f32(
    target: BufferType,
    src_data: &[f32],
    usage: UsageType,
    src_offset: u32,
    length: Option<u32>,
) {
    buffer_data_f32_sys(
        target as u32,
        src_data,
        usage as u32,
        src_offset,
        length.unwrap_or(0),
    );
}

/// Binds the buffer currently bound to `BufferType::ArrayBuffer` to a generic
/// vertex attribute of the current vertex buffer object and specifies its
/// layout.
#[inline]
pub fn vertex_attr_ptr(
    index: u32,
    size: i32,
    data_type: DataType,
    normalized: bool,
    stride: i32,
    offset: i32,
) {
    vertex_attr_ptr_sys(
        index,
        size,
        data_type as u32,
        normalized,
        stride,
        offset,
    );
}

/// Renders the specified primitive type using array data.
#[inline]
pub fn draw_arrays(mode: RenderingPrimitive, first: i32, count: i32) {
    draw_arrays_sys(mode as u32, first, count);
}

/// Renders the specified primitive type using array data. Used for index-based
/// rendering.
#[inline]
pub fn draw_elements(
    mode: RenderingPrimitive,
    count: i32,
    data_type: ElementDataType,
    offset: i32,
) {
    draw_elements_sys(mode as u32, count, data_type as u32, offset);
}

/// Enable a WebGL capability.
#[inline]
pub fn enable(capability: Capability) {
    enable_sys(capability as u32);
}
