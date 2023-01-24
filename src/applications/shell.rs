
use async_trait::async_trait;
use lazy_static::lazy_static;
use crate::{println, print, alloc::string::ToString};
use alloc::{string::String, vec::Vec, boxed::Box, rc::Rc};
use spin::Mutex;
use crate::applications::{
	calc::Calculator,
	rickroll::Rickroll,
	crystalfetch::CrystalFetch,
};
use crate::tasks::keyboard::ScanCodeStream;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use futures_util::stream::StreamExt;
use crate::alloc::borrow::ToOwned;
use crate::vga_buffer::{write, Color, WRITER};
use x86_64::instructions::interrupts;

lazy_static! {
	pub static ref CMD: Mutex<CommandHandler> = Mutex::new(CommandHandler::new());
}


/// boilerplate function
/// may provide other interfacing options later on idk.
pub async fn command_handler() {
	eventloop().await;
}


/// this function starts the shell running, the function will loop repeatedly until the command to shutdown
/// TODO: implement shutdown command
pub async fn eventloop() {
	println!("running!");

	let mut fetch = CrystalFetch::new();
	fetch.run(String::from("e")).await;
	
	CMD.lock().prompt();		
	
	loop {
		let string = CMD.lock().get_string().await;
		CMD.lock().current.push_str(&string);
		exec().await;
		CMD.lock().prompt();
	}
}

async fn exec() -> Result<(), Error> {
	let mut command = false;
	let mut current = CMD.lock().current.clone();
	
	CMD.lock().history.history.push(current.clone());

	current.pop();
	CMD.lock().current = String::new();



	let (cmd, args) = match current.split_once(" ") {
		Some((x,y)) => { command = true; (x,y.to_string()) },
		None => ("none", "none".to_string()),
	};

	
	if command == true {
		match cmd {
			"calculate"|"calc"|"solve" => {
				let mut cmd = Calculator::new();
				cmd.run(args).await;
			},
			"echo" => { println!("Crystal: '{}'", args) },
			
			"rickroll" => {
				let mut cmd = Rickroll::new();
				cmd.run(args).await;
			}
			
			"crystalfetch" => {
				let mut cmd = CrystalFetch::new();
				cmd.run(args).await;
			}
			
			_ => { println!("this command has not been implemented yet!"); },
		};
	} else {
		println!("this command does not exist! (or too few arguments supplied)")
	}

	
	Ok(())
}





pub struct CommandHandler {
	current: String,
	history: CmdHistory,
	scancodes: ScanCodeStream,
	keyboard: Keyboard<layouts::Uk105Key, ScancodeSet1>,
}

impl CommandHandler {
	pub fn new() -> Self {
		let handler = Self {
			current: String::new(),
			history: CmdHistory { history: Vec::new() },
			scancodes: ScanCodeStream::new(),
			keyboard: Keyboard::new(layouts::Uk105Key, ScancodeSet1, HandleControl::Ignore),
		};
		handler
	}


		
	// this function is activated every time the user presses a key on the keyboard
	// it accesses the queue of keys (a static ref in src/tasks/keyboard.rs)

	pub async fn get_keystroke(&mut self) -> char {
		loop {
			if let Some(scancode) = self.scancodes.next().await {
				if let Ok(Some(key_event)) = self.keyboard.add_byte(scancode) {
					if let Some(key) = self.keyboard.process_keyevent(key_event) {
						match key {
							DecodedKey::Unicode(character) => { 
								if character == '\b' {
									WRITER.lock().backspace();
									continue;
								} else {
									return character;
								}
							},
							DecodedKey::RawKey(key) => { print!("{:?}", key) },
						}
					}
				}
			}			
		}
	}

	pub async fn get_string(&mut self) -> String {
		let mut val = String::new();
		loop {
			let character = self.get_keystroke().await;
			print!("{}", character);
			let (character, execute): (char, bool) = match character {
				'\n' => (character, true),
				_ => (character, false),
			};	
			val.push(character);
			if execute {
				return val;
			} 			
		}

	}

	// displays a text prompt for the user to type into.
	// this is a separate function so that it can be developed as necessary later on
	// TODO: coloured prompt
	
	pub fn prompt(&self) {
		print!("\n [ Crystal ] >> ");
	}
	
	
	// this function is run every time the enter key is pressed in the command line mode.
	// it detects the command that is being run and then executes it, passing the arguments to it.

}



struct CmdHistory {
	history: Vec<String>,
}

#[derive(Debug)]
pub enum Error {
	UnknownCommand(String),
	CommandFailed(String),
}

#[async_trait]
pub trait Application {
	fn new() -> Self;

	async fn input(&mut self) -> String;

	async fn keystroke(&mut self) -> char;

	async fn run(&mut self, args: String) -> Result<(), Error> {
		Ok(())
	}
}
