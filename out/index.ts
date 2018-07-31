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

export function buffer_data_u16_sys(target:     number,
                                    src_data:   Uint16Array,
                                    usage:      number,
                                    src_offset: number,
                                    length:     number): void {
    gl.bufferData(target, src_data, usage, src_offset, length);
}

export function buffer_data_f32_sys(target:     number,
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

export function draw_elements_sys(mode:      number,
                                  count:     number,
                                  data_type: number,
                                  offset:    number): void {
    gl.drawElements(mode, count, data_type, offset);
}

export function uniform2f(loc: WebGLUniformLocation,
                          x:   number,
                          y:   number): void {
    gl.uniform2f(loc, x, y);
}

export function uniform3fv(loc:  WebGLUniformLocation,
                           data: Float32Array): void {
    gl.uniform3fv(loc, data);
}

export function uniform4fv(loc:  WebGLUniformLocation,
                           data: Float32Array): void {
    gl.uniform4fv(loc, data);
}

export function uniform_matrix4fv(loc:  WebGLUniformLocation,
                                  data: Float32Array): void {
    gl.uniformMatrix4fv(loc, false, data);
}

export function enable_sys(cap: number): void {
    gl.enable(cap);
}

export function get_seed(): Uint32Array {
    const seed_len = 4;
    const seed = new Uint32Array(seed_len);
    window.crypto.getRandomValues(seed);

    return seed;
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

    bg.init();

    const trans_x_input = document.getElementById("trans_x");
    const trans_y_input = document.getElementById("trans_y");
    const trans_z_input = document.getElementById("trans_z");

    const lookat_eye_x_input = document.getElementById("lookat_eye_x");
    const lookat_eye_y_input = document.getElementById("lookat_eye_y");
    const lookat_eye_z_input = document.getElementById("lookat_eye_z");

    const rot_x_input = document.getElementById("rot_x");
    const rot_y_input = document.getElementById("rot_y");
    const rot_z_input = document.getElementById("rot_z");

    if (!(trans_x_input      instanceof HTMLInputElement &&
          trans_y_input      instanceof HTMLInputElement &&
          trans_z_input      instanceof HTMLInputElement &&
          lookat_eye_x_input instanceof HTMLInputElement &&
          lookat_eye_y_input instanceof HTMLInputElement &&
          lookat_eye_z_input instanceof HTMLInputElement &&
          rot_x_input        instanceof HTMLInputElement &&
          rot_y_input        instanceof HTMLInputElement &&
          rot_z_input        instanceof HTMLInputElement)) {
        throw new Error("Missing controls");
    }

    const update = () =>
        bg.render(
            +trans_x_input.value,
            +trans_y_input.value,
            +trans_z_input.value,
            +lookat_eye_x_input.value,
            +lookat_eye_y_input.value,
            +lookat_eye_z_input.value,
            +rot_x_input.value,
            +rot_y_input.value,
            +rot_z_input.value
        );

    update();

    trans_x_input.addEventListener("input", update);
    trans_y_input.addEventListener("input", update);
    trans_z_input.addEventListener("input", update);
    lookat_eye_x_input.addEventListener("input", update);
    lookat_eye_y_input.addEventListener("input", update);
    lookat_eye_z_input.addEventListener("input", update);
    rot_x_input.addEventListener("input", update);
    rot_y_input.addEventListener("input", update);
    rot_z_input.addEventListener("input", update);
})
.catch(e => {
    log(`Error resolving promise \`webgl_test\`: ${e}`);
    console.log(e);
});
