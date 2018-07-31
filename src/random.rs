use pcg_rand;
use rand::{Rng, SeedableRng};
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

lazy_static! {
    static ref ENGINE: Mutex<pcg_rand::Pcg32> =
        Mutex::new(pcg_rand::Pcg32::from_seed({
            let seed = get_seed();
            let s0 = seed[0] as u64 | ((seed[1] as u64) << 32);
            let s1 = seed[2] as u64 | ((seed[3] as u64) << 32);

            [s0, s1]
        }));
}

#[wasm_bindgen(module = "./index")]
extern "C" {
    /// Gets 16 cryptographic-strength bytes of randomness (the resulting
    /// array has `len`gth `4`).
    fn get_seed() -> Box<[u32]>;
}

/// Return the next random `f32` selected from the half-open interval `[0, 1)`.
#[inline]
pub fn next_f32() -> f32 {
    match ENGINE.lock() {
        Ok(mut engine) => engine.next_f32(),
        _ => unreachable!(),
    }
}
