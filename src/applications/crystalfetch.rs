use async_trait::async_trait;
use crate::applications::shell::{
	CMD, 
	Application,
	Error,
};
use crate::os::OS;
use crate::{print, println};
use alloc::{boxed::Box, string::String, vec::Vec};
use core::iter::repeat;
use crate::vga_buffer::{Color, write};

pub struct CrystalFetch {}

#[async_trait]
impl Application for CrystalFetch {


	fn new() -> Self {
		Self {}
	}
	async fn input(&mut self) -> String {
		String::from("this does nothing")
	}
	async fn keystroke(&mut self) -> char {
		'e'
	}

	
	async fn run(&mut self, args:String) -> Result<(), Error> {

		let os = OS.lock().os.clone();
		let version = OS.lock().version.clone();
	
		write(format_args!("
   --------------------------------------
   
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
       |  OS     ->  {}
       |  BUILD  ->  {}
       |  Host   ->  ArchLinux-QEMU  
       |  RAM    ->  idk              
       |  Shell  ->  CrystalSH        
       |  API    ->  CrystalAPI       
       |  Pkgs   ->  4                
       |  Fetch  ->  CrystalFetch

   ---------------------------------------
", os, version);

		println!("{}", "\n");
		
		Ok(())
	}
	
}
