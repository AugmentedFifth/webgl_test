use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "./index")]
extern "C" {
    /// `console.log(msg);`
    pub fn log(msg: &str);
}
