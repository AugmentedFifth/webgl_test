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

    /// https://developer.mozilla.org/en-US/docs/Web/API/WebGLTexture
    pub type WebGLTexture;
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
    pub fn uniform_matrix3fv(loc: &WebGLUniformLocation, data: &[f32]);

    /// Specifies the value of a uniform.
    pub fn uniform_matrix4fv(loc: &WebGLUniformLocation, data: &[f32]);

    /// Specifies the value of a uniform.
    pub fn uniform1i(loc: &WebGLUniformLocation, x: i32);

    fn enable_sys(cap: u32);

    pub fn create_texture() -> WebGLTexture;

    fn active_texture_sys(texture_ix: u32);

    fn bind_texture_sys(target: u32, texture: &WebGLTexture);

    fn tex_image_2d_u8_sys(
        target: u32,
        level: i32,
        internal_format: u32,
        width: i32,
        height: i32,
        format: u32,
        src_data: &[u8],
    );

    fn tex_image_2d_u16_sys(
        target: u32,
        level: i32,
        internal_format: u32,
        width: i32,
        height: i32,
        format: u32,
        src_data: &[u16],
    );

    fn pixel_storei_sys(pname: u32, param: i32);

    fn tex_parameteri_sys(target: u32, pname: u32, param: i32);

    pub fn depth_mask(flag: bool);

    fn depth_func_sys(func: u32);
}

pub const DEPTH_BUFFER_BIT: u32 = 0x0000_0100;
pub const STENCIL_BUFFER_BIT: u32 = 0x0000_0400;
pub const COLOR_BUFFER_BIT: u32 = 0x0000_4000;

pub const NEAREST: i32 = 0x2600;
pub const LINEAR: i32 = 0x2601;
pub const REPEAT: i32 = 0x2901;
pub const CLAMP_TO_EDGE: i32 = 0x812F;
pub const MIRRORED_REPEAT: i32 = 0x8370;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ShaderType {
    FragmentShader = 0x8B30,
    VertexShader = 0x8B31,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ElementDataType {
    UnsignedByte = 0x1401,
    UnsignedShort = 0x1403,
    UnsignedInt = 0x1405,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum TextureIndex {
    Texture0 = 0x84C0,
    Texture1 = 0x84C1,
    Texture2 = 0x84C2,
    Texture3 = 0x84C3,
    Texture4 = 0x84C4,
    Texture5 = 0x84C5,
    Texture6 = 0x84C6,
    Texture7 = 0x84C7,
    Texture8 = 0x84C8,
    Texture9 = 0x84C9,
    Texture10 = 0x84CA,
    Texture11 = 0x84CB,
    Texture12 = 0x84CC,
    Texture13 = 0x84CD,
    Texture14 = 0x84CE,
    Texture15 = 0x84CF,
    Texture16 = 0x84D0,
    Texture17 = 0x84D1,
    Texture18 = 0x84D2,
    Texture19 = 0x84D3,
    Texture20 = 0x84D4,
    Texture21 = 0x84D5,
    Texture22 = 0x84D6,
    Texture23 = 0x84D7,
    Texture24 = 0x84D8,
    Texture25 = 0x84D9,
    Texture26 = 0x84DA,
    Texture27 = 0x84DB,
    Texture28 = 0x84DC,
    Texture29 = 0x84DD,
    Texture30 = 0x84DE,
    Texture31 = 0x84DF,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum TextureTarget {
    Texture2d = 0x0DE1,
    Texture3d = 0x806F,
    TextureCubeMap = 0x8513,
    Texture2dArray = 0x8C1A,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum TextureBindPoint {
    Texture2d = 0x0DE1,
    TextureCubeMapPositiveX = 0x8515,
    TextureCubeMapNegativeX = 0x8516,
    TextureCubeMapPositiveY = 0x8517,
    TextureCubeMapNegativeY = 0x8518,
    TextureCubeMapPositiveZ = 0x8519,
    TextureCubeMapNegativeZ = 0x851A,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ColorFormat {
    Alpha = 0x1906,
    RGB = 0x1907,
    RGBA = 0x1908,
    Luminance = 0x1909,
    LuminanceAlpha = 0x190A,
    R8 = 0x8229,
    RG8 = 0x822B,
    R16f = 0x822D,
    R32f = 0x822E,
    RG16f = 0x822F,
    RG32f = 0x8230,
    R8i = 0x8231,
    R8ui = 0x8232,
    R16i = 0x8233,
    R16ui = 0x8234,
    R32i = 0x8235,
    R32ui = 0x8236,
    RG8i = 0x8237,
    RG8ui = 0x8238,
    RG16i = 0x8239,
    RG16ui = 0x823A,
    RG32i = 0x823B,
    RG32ui = 0x823C,
    RGBA32f = 0x8814,
    RGB32f = 0x8815,
    RGBA16f = 0x881A,
    RGB16f = 0x881B,
    SRGB8 = 0x8C41,
    SRGB8Alpha8 = 0x8C43,
    R11fG11fB10f = 0x8C3A,
    RGB9E5 = 0x8C3D,
    RGBA32ui = 0x8D70,
    RGB32ui = 0x8D71,
    RGBA16ui = 0x8D76,
    RGB16ui = 0x8D77,
    RGBA8ui = 0x8D7C,
    RGB8ui = 0x8D7D,
    RedInteger = 0x8D94,
    RGBInteger = 0x8D98,
    RGBAInteger = 0x8D99,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum PixelStoreParam {
    UnpackAlignment = 0x0CF5,
    PackAlignment = 0x0D05,
    UnpackFlipYWebgl = 0x9240,
    UnpackPremultiplyAlphaWebgl = 0x9241,
    UnpackColorspaceConversionWebgl = 0x9243,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum TextureParam {
    TextureMagFilter = 0x2800,
    TextureMinFilter = 0x2801,
    TextureWrapS = 0x2802,
    TextureWrapT = 0x2803,
    TextureWrapR = 0x8072,
    TextureMinLod = 0x813A,
    TextureMaxLod = 0x813B,
    TextureBaseLevel = 0x813C,
    TextureMaxLevel = 0x813D,
    TextureCompareMode = 0x884C,
    TextureCompareFunc = 0x884D,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum DepthFunc {
    Never = 0x0200,
    Less = 0x0201,
    Equal = 0x0202,
    LEqual = 0x0203,
    Greater = 0x0204,
    NotEqual = 0x0205,
    GEqual = 0x0206,
    Always = 0x0207,
}

impl TextureIndex {
    #[inline]
    pub fn as_index(&self) -> u32 {
        *self as u32 - TextureIndex::Texture0 as u32
    }
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

#[inline]
pub fn active_texture(texture_ix: TextureIndex) {
    active_texture_sys(texture_ix as u32);
}

#[inline]
pub fn bind_texture(target: TextureTarget, texture: &WebGLTexture) {
    bind_texture_sys(target as u32, texture);
}

#[inline]
pub fn tex_image_2d_u8(
    target: TextureBindPoint,
    level: i32,
    internal_format: ColorFormat,
    width: i32,
    height: i32,
    format: ColorFormat,
    src_data: &[u8],
) {
    tex_image_2d_u8_sys(
        target as u32,
        level,
        internal_format as u32,
        width,
        height,
        format as u32,
        src_data,
    );
}

#[inline]
pub fn tex_image_2d_u16(
    target: TextureBindPoint,
    level: i32,
    internal_format: ColorFormat,
    width: i32,
    height: i32,
    format: ColorFormat,
    src_data: &[u16],
) {
    tex_image_2d_u16_sys(
        target as u32,
        level,
        internal_format as u32,
        width,
        height,
        format as u32,
        src_data,
    );
}

#[inline]
pub fn pixel_storei(pname: PixelStoreParam, param: i32) {
    pixel_storei_sys(pname as u32, param);
}

#[inline]
pub fn tex_parameteri(target: TextureTarget, pname: TextureParam, param: i32) {
    tex_parameteri_sys(target as u32, pname as u32, param);
}

#[inline]
pub fn depth_func(func: DepthFunc) {
    depth_func_sys(func as u32);
}
