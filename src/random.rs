use pcg_rand;
use rand::{
    distributions::{uniform::Uniform, Distribution},
    SeedableRng,
};
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

lazy_static! {
    static ref ENGINE: Mutex<pcg_rand::Pcg32> =
        Mutex::new(pcg_rand::Pcg32::from_seed({
            let seed = get_seed();
            let seed = u64::from(seed[0]) | (u64::from(seed[1]) << 32);

            pcg_rand::seeds::PcgSeeder::seed(seed)
        }));
}

#[wasm_bindgen(module = "./index")]
extern "C" {
    /// Gets 8 cryptographic-strength bytes of randomness (the resulting array
    /// has `len`gth `2`).
    fn get_seed() -> Box<[u32]>;
}

/// Return the next random `f32` selected from the half-open interval `[0, 1)`.
#[inline]
pub fn next_f32() -> f32 {
    lazy_static! {
        static ref dist: Uniform<f32> = Uniform::new(0.0, 1.0);
    }

    match ENGINE.lock() {
        Ok(mut engine) => dist.sample(&mut *engine),
        _ => unreachable!(),
    }
}
