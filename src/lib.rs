#![feature(use_extern_macros)]

mod geometry;
mod js;
mod mains;
mod random;
mod webgl;

pub use js::*;
pub use mains::*;
pub use random::*;
pub use webgl::*;

#[macro_use]
extern crate lazy_static;
extern crate nalgebra as na;
extern crate pcg_rand;
extern crate rand;
extern crate wasm_bindgen;
