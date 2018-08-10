use wasm_bindgen::prelude::*;

#[repr(u8)]
pub enum EventType {
    KeyDown = 0x01,
    KeyUp = 0x02,
    MouseMove = 0x03,
}

#[wasm_bindgen(module = "./index")]
extern "C" {
    /// `console.log(msg);`
    pub fn log(msg: &str);

    /// https://developer.mozilla.org/en-US/docs/Web/API/Performance/now
    pub fn now() -> f64;
}

#[wasm_bindgen(module = "./event")]
extern "C" {
    pub type EventQueue;
    pub type Event;

    #[wasm_bindgen(method)]
    pub fn get(this: &EventQueue, index: u32) -> Event;

    #[wasm_bindgen(method, getter)]
    pub fn len(this: &EventQueue) -> u32;

    #[wasm_bindgen(method, getter)]
    pub fn opcode(this: &Event) -> u8;

    #[wasm_bindgen(method, getter)]
    pub fn payload(this: &Event) -> Box<[u8]>;
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

impl EventType {
    pub fn from_u8(n: u8) -> Option<Self> {
        match n {
            n if n == EventType::KeyDown as u8 => Some(EventType::KeyDown),
            n if n == EventType::KeyUp as u8 => Some(EventType::KeyUp),
            n if n == EventType::MouseMove as u8 => Some(EventType::MouseMove),
            _ => None,
        }
    }
}
