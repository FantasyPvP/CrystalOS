use async_trait::async_trait;
use futures_util::future::OrElse;
use lazy_static::lazy_static;
use spin::Mutex;

use crate::shell::{
	Application,
	Error,
    CMD
};
use crate::os::OS;
use crate::{println};
use alloc::{boxed::Box, string::{String, ToString}, vec::Vec};
use crate::vga_buffer::{Color, write};

use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use rand::RngCore;

// this is where the standard library for the operating system will be defined
// my aim is to completely separate this from the shell.

// these functions should all be asynchronous.

lazy_static! {
    pub static ref RANDOM: Mutex<SmallRng> = Mutex::new(SmallRng::seed_from_u64(1));
}

pub async fn stdin() -> String {
    let string = CMD.lock().get_string().await;
    string
}

pub async fn stdchar() -> char {
    let chr = CMD.lock().get_keystroke().await;
    chr
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
