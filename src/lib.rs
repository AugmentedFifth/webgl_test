#![feature(use_extern_macros)]

mod geometry;
mod js;
mod mains;
mod webgl;

pub use js::*;
pub use mains::*;
pub use webgl::*;

extern crate nalgebra as na;
extern crate wasm_bindgen;
