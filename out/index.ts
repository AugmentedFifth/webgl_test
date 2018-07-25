"use strict";

const webgl_test = import("./webgl_test");

//////// Globals     ////////
let gl: WebGL2RenderingContext;
//////// End globals ////////

export function log(msg: string): void {
    console.log(msg);
}

export function create_shader_sys(type: number,
                                  src:  string): WebGLShader | undefined {
    const shader = gl.createShader(type);
    if (shader === null) {
        return undefined;
    }
    gl.shaderSource(shader, src);
    gl.compileShader(shader);

    const success = gl.getShaderParameter(shader, gl.COMPILE_STATUS);
    if (success) {
        return shader;
    }

    log(`${gl.getShaderInfoLog(shader)}`);
    gl.deleteShader(shader);

    return undefined;
}

export function create_program(vertex_shader:   WebGLShader,
                               fragment_shader: WebGLShader): WebGLProgram |
                                                              undefined {
    const program = gl.createProgram();
    if (program === null) {
        return undefined;
    }
    gl.attachShader(program, vertex_shader);
    gl.attachShader(program, fragment_shader);
    gl.linkProgram(program);

    const success = gl.getProgramParameter(program, gl.LINK_STATUS);
    if (success) {
        return program;
    }

    log(`${gl.getProgramInfoLog(program)}`);
    gl.deleteProgram(program);

    return undefined;
}

export function get_uniform_location(prog: WebGLProgram,
                                     name: string): WebGLUniformLocation |
                                                    undefined {
    const uni_loc = gl.getUniformLocation(prog, name);

    return uni_loc === null ? undefined : uni_loc;
}

export function create_buffer(): WebGLBuffer {
    const buffer = gl.createBuffer();
    if (buffer === null) {
        throw new Error("WebGLRenderingContext.createBuffer() failed");
    }

    return buffer;
}

export function get_attr_location(prog: WebGLProgram, name: string): number {
    return gl.getAttribLocation(prog, name);
}

export function bind_buffer_sys(target: number, buffer: WebGLBuffer): void {
    gl.bindBuffer(target, buffer);
}

export function buffer_data_sys(target:     number,
                                src_data:   Uint8Array,
                                usage:      number,
                                src_offset: number,
                                length:     number): void {
    gl.bufferData(target, src_data, usage, src_offset, length);
}

export function buffer_data_sys_f32(target:     number,
                                    src_data:   Float32Array,
                                    usage:      number,
                                    src_offset: number,
                                    length:     number): void {
    gl.bufferData(target, src_data, usage, src_offset, length);
}

export function create_vertex_array(): WebGLVertexArrayObject {
    const vao = gl.createVertexArray();
    if (vao === null) {
        throw new Error("Failed to create a VAO");
    }

    return vao;
}

export function bind_vertex_array(vao: WebGLVertexArrayObject): void {
    gl.bindVertexArray(vao);
}

export function enable_vertex_attr_array(index: number): void {
    gl.enableVertexAttribArray(index);
}

export function vertex_attr_ptr_sys(index:      number,
                                    size:       number,
                                    data_type:  number,
                                    normalized: boolean,
                                    stride:     number,
                                    offset:     number): void {
    gl.vertexAttribPointer(index, size, data_type, normalized, stride, offset);
}

export function get_canvas_width(): number {
    return gl.canvas.width;
}

export function get_canvas_height(): number {
    return gl.canvas.height;
}

export function resize_canvas_to_display(): void {
    if (
        gl.canvas.width !== gl.canvas.clientWidth ||
        gl.canvas.height !== gl.canvas.clientHeight
    ) {
        gl.canvas.width = gl.canvas.clientWidth;
        gl.canvas.height = gl.canvas.clientHeight;
    }
}

export function reset_viewport(): void {
    gl.viewport(0, 0, gl.canvas.width, gl.canvas.height);
}

export function clear_color(r: number, g: number, b: number, a: number): void {
    gl.clearColor(r, g, b, a);
}

export function clear(mask: number): void {
    gl.clear(mask);
}

export function use_program(prog: WebGLProgram): void {
    gl.useProgram(prog);
}

export function draw_arrays_sys(mode:  number,
                                first: number,
                                count: number): void {
    gl.drawArrays(mode, first, count);
}

export function uniform2f(loc: WebGLUniformLocation,
                          x:   number,
                          y:   number): void {
    gl.uniform2f(loc, x, y);
}

export function uniform4fv(loc:  WebGLUniformLocation,
                           data: Float32Array): void {
    gl.uniform4fv(loc, data);
}

export function uniform_matrix4fv(loc:  WebGLUniformLocation,
                                  data: Float32Array): void {
    gl.uniformMatrix4fv(loc, false, data);
}

webgl_test.then(bg => {
    const canvas = document.getElementById("c");
    if (!(canvas instanceof HTMLCanvasElement)) {
        throw new Error("No HTMLCanvasElement with the ID \"c\"");
    }

    const gl_ctx = canvas.getContext("webgl2");
    if (!(gl_ctx instanceof WebGL2RenderingContext)) {
        throw new Error("No WebGL2 support detected");
    }

    gl = gl_ctx;

    bg.test0();
})
.catch(e => log(`Error resolving promise \`webgl_test\`: ${e}`));
