#![feature(use_extern_macros, wasm_custom_section, wasm_import_module)]

mod geometry;
mod js;
mod mains;
mod webgl;

pub use js::*;
pub use mains::*;
pub use webgl::*;

extern crate nalgebra as na;
extern crate wasm_bindgen;
