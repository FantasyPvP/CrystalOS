use async_trait::async_trait;
use alloc::{boxed::Box, string::String, vec::Vec};

use crate::{
    kernel::{
        os::OS,
        render::{Color, write}
    },
    println,
    std::application::{
        Application,
        Error,
    },
};


pub struct CrystalFetch {}

#[async_trait]
impl Application for CrystalFetch {

	fn new() -> Self {
		Self {}
	}

	async fn run(&mut self, _args: Vec<String>) -> Result<(), Error> {

		let os = OS.lock().os.clone();
		let version = OS.lock().version.clone();


		write(format_args!("
────────────────────────────────────────────────────────
    _____                _        _  ____   _____
   / ____|              | |      | |/ __ \\ / ____|
  | |     _ __ _   _ ___| |_ __ _| | |  | | (___
  | |    | '__| | | / __| __/ _` | | |  | |\\___ \\
  | |____| |  | |_| \\__ \\ || (_| | | |__| |____) |
   \\_____|_|   \\__, |___/\\__\\__,_|_|\\____/|_____/
                __/ |
               |___/
"), (Color::Magenta, Color::Black));

		println!("
       ╔═══════════════════════════════
       ║
       ║   OS      »  {}
       ║   BUILD   »  {}
       ║   RAM     »  idk
       ║   Shell   »  CrystalSH
       ║   API     »  CrystalAPI
       ║   Pkgs    »  4
       ║   Fetch   »  CrystalFetch
       ║
       ╚═══════════════════════════════

────────────────────────────────────────────────────────
", os, version);
		Ok(())
        
	}

}
