const bindgen = import("./webgl_test");

bindgen.then(bg => {
    bg.greet("world");
})
.catch(e => console.log(e));
