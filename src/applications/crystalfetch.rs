use async_trait::async_trait;
use crate::applications::shell::{
	CMD, 
	Application,
	Error,
};
use crate::{print, println};
use alloc::{boxed::Box, string::String, vec::Vec};
use core::iter::repeat;


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
	
		println!("
   --------------------------------------
   
    _____                _        _  ____   _____ 
   / ____|              | |      | |/ __ \\ / ____|
  | |     _ __ _   _ ___| |_ __ _| | |  | | (___  
  | |    | '__| | | / __| __/ _` | | |  | |\\___ \\ 
  | |____| |  | |_| \\__ \\ || (_| | | |__| |____) |
   \\_____|_|   \\__, |___/\\__\\__,_|_|\\____/|_____/ 
                __/ |                             
               |___/                              

       |  OS     ->  CrystalOS-ALPHA  
       |  BUILD  ->  0.1.1            
       |  Host   ->  ArchLinux-QEMU  
       |  RAM    ->  idk              
       |  Shell  ->  CrystalSH        
       |  API    ->  CrystalAPI       
       |  Pkgs   ->  4                
       |  Fetch  ->  CrystalFetch

   ---------------------------------------

");
		println!("{}", "\n".repeat(1));
		
		Ok(())
	}
	
}
