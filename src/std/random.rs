
use alloc::{boxed::Box, string::{String, ToString}, vec::Vec};
use rand::{Rng, SeedableRng, rngs::SmallRng, RngCore};
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref RANDOM: Mutex<SmallRng> = Mutex::new(SmallRng::seed_from_u64(1));
}




pub struct Random;

impl Random {
    pub fn int(lower: usize, upper: usize) -> usize {
        loop {
            let integer: u64 = RANDOM.lock().next_u64();
            let mut integer: String = integer.to_string();
            integer = "0".repeat(20 - integer.len()) + &integer;
            let integer: usize = integer[1..upper.to_string().len() + 1].parse().unwrap();
            if integer <= upper && integer >= lower {
                return integer;
            } else {
                continue;
            }
        }

    }
    pub fn selection<T: Clone>(ls: Vec<T>) -> T {
        let range = Random::int(0, ls.len() - 1);
        ls[range as usize].clone()
    }
}