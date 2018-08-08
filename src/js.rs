use wasm_bindgen::prelude::*;

pub const KEY_DOWN: u8 = 0x01;
pub const KEY_UP: u8 = 0x02;

pub mod key {
    pub const W: u8 = 0x01;
    pub const A: u8 = 0x02;
    pub const S: u8 = 0x03;
    pub const D: u8 = 0x04;
}

#[wasm_bindgen(module = "./index")]
extern "C" {
    /// `console.log(msg);`
    pub fn log(msg: &str);

    /// https://developer.mozilla.org/en-US/docs/Web/API/Performance/now
    pub fn now() -> f64;
}

#[wasm_bindgen(module = "./vec")]
extern "C" {
    pub type Uint16Vec;

    #[wasm_bindgen(method)]
    pub fn clear(this: &Uint16Vec);

    #[wasm_bindgen(method)]
    pub fn get(this: &Uint16Vec, index: u32) -> u16;

    #[wasm_bindgen(method)]
    pub fn peek(this: &Uint16Vec) -> u16;

    #[wasm_bindgen(method)]
    pub fn pop(this: &Uint16Vec) -> u16;

    #[wasm_bindgen(method)]
    pub fn push(this: &Uint16Vec, b: u16);

    #[wasm_bindgen(method, getter)]
    pub fn data(this: &Uint16Vec) -> Box<[u16]>;

    #[wasm_bindgen(method, getter)]
    pub fn len(this: &Uint16Vec) -> u32;
}
