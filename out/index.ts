const webgl_test = import("./webgl_test");

export function log(msg: string): void {
    console.log(msg);
}

webgl_test.then(bg => {
    bg.test0("world");
})
.catch(log);
