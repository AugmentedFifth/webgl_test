use pcg_rand::Pcg32;
use rand::{
    distributions::{Distribution, Standard},
    FromEntropy,
    Rng,
};
use std::cell::RefCell;

thread_local! {
    static RNG: RefCell<Pcg32> = RefCell::new(Pcg32::from_entropy());
}

pub fn gen<T>() -> T
where
    Standard: Distribution<T>,
{
    RNG.with(|rng| rng.borrow_mut().gen())
}
