use async_trait::async_trait;
use alloc::{boxed::Box, string::String, vec::Vec};

use crate::{
    println,
    std::{application::{
        Application,
        Error,
    }, render::Window},
};

const GIGACHAD: [&'static str; 3] = ["fantasypvp", "zxq5", "ZXQ5"];

pub struct GigachadDetector {}

#[async_trait]
impl Application for GigachadDetector {
    fn new() -> Self {
        Self {}
    }

    async fn run(&mut self, window: Option<Window>, args: Vec<String>) -> Result<(), Error> {
        for arg in args {
            self.detect_gigachad_by_username(&arg)
        }
        Ok(())
    }
}

impl GigachadDetector {
    pub fn detect_gigachad_by_username(&self, username: &str) {
        if GIGACHAD.contains(&username) {
            println!("{} is a gigachad B'YES", username);
            println!("
    /$$ /$$$$$$$$ /$$   /$$  /$$$$$$  /$$$$$$$       /$$ /$$  /$$
   /$$/|_____ $$ | $$  / $$ /$$__  $$| $$____/      /$$/|  $$|  $$
  /$$/      /$$/ |  $$/ $$/| $$  \\ $$| $$          /$$/  \\  $$\\  $$
 /$$/      /$$/   \\  $$$$/ | $$  | $$| $$$$$$$    /$$/    \\  $$\\  $$
|  $$     /$$/     >$$  $$ | $$  | $$|_____  $$  /$$/      /$$/ /$$/
 \\  $$   /$$/     /$$/\\  $$| $$/$$ $$ /$$  \\ $$ /$$/      /$$/ /$$/
  \\  $$ /$$$$$$$$| $$  \\ $$|  $$$$$$/|  $$$$$$//$$/      /$$/ /$$/
   \\__/|________/|__/  |__/ \\____ $$$ \\______/|__/      |__/ |__/
                                 \\__/
    ")
        } else {
            println!("{} is not a gigachad", username);
        }
    }
}