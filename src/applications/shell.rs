use async_trait::async_trait;
use lazy_static::lazy_static;
use crate::{println, print, alloc::string::ToString};
use alloc::{string::String, vec::Vec, boxed::Box, rc::Rc};
use spin::Mutex;
use crate::applications::calc;
use crate::tasks::keyboard::ScanCodeStream;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use futures_util::stream::StreamExt;

lazy_static! {
	pub static ref CMD: Mutex<CommandHandler> = Mutex::new(CommandHandler::new());
}

pub async fn command_handler() {
	let mut cmd = CMD.lock();
	cmd.run().await;
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

	// this function starts the shell running, the function will loop repeatedly until the command to shutdown
	// TODO: implement shutdown command
		
	pub async fn run(&mut self) {
		println!("running!");
		self.prompt();
		
		loop {
			let character = self.get_keystroke().await;
			print!("{}", &character);
			let (character, execute): (char, bool) = match character {
				'\n' => (character, true),
				_ => (character, false),
			};
			self.current.push(character);
			
			if execute {
				self.execute().await;
				self.prompt();
			}
		}
	}

	// this function is activated every time the user presses a key on the keyboard
	// it accesses the queue of keys (a static ref in src/tasks/keyboard.rs)

	async fn get_keystroke(&mut self) -> char {
		loop {
			if let Some(scancode) = self.scancodes.next().await {
				if let Ok(Some(key_event)) = self.keyboard.add_byte(scancode) {
					if let Some(key) = self.keyboard.process_keyevent(key_event) {
						match key {
							DecodedKey::Unicode(character) => return character,
							DecodedKey::RawKey(key) => print!("{:?}", key),
						}
					}
				}
			}			
		}
	}

	// displays a text prompt for the user to type into.
	// this is a separate function so that it can be developed as necessary later on
	// TODO: coloured prompt
	
	fn prompt(&self) {
		print!(" Crystal>> ");
	}
	
	
	// this function is run every time the enter key is pressed in the command line mode.
	// it detects the command that is being run and then executes it, passing the arguments to it.
	async fn execute(&mut self) -> Result<(), Error> {
		let mut command = false;

		self.history.history.push(self.current.clone());
		let mut current = self.current.clone();
		current.pop();
		self.current = String::new();

		let (cmd, args) = match current.split_once(" ") {
			Some((x,y)) => { command = true; (x,y.to_string()) },
			None => ("none", "none".to_string()),
		};

		if command == true {
			match cmd {
				"calculate"|"calc"|"solve" => { calc::calculate(args.to_string()); },
				"echo" => { println!("Crystal: '{}'", args) },
				"rickroll" => {
					let mut cmd = Rickroll::new();
					cmd.run(self, args).await;
				}
				_ => { println!("this command has not been implemented yet!"); },
			};
		} else {
			println!("this command does not exist! (or too few arguments supplied)")
		}
		Ok(())
	}
}



struct CmdHistory {
	history: Vec<String>,
}

pub enum Error {
	UnknownCommand(String),
	CommandFailed(String),
}

#[async_trait]
pub trait Application {
	fn new() -> Self;

//	async fn input(&mut self) -> String;

	async fn run(&mut self, handler: &mut CommandHandler, args: String) -> Result<(), Error> {
		
		Ok(())
	}
}


struct Rickroll {}

#[async_trait]
impl Application for Rickroll {
	fn new() -> Self {
		Self {}
	}

//	async fn input(&mut self) -> String {
//		handler.get_keystroke().await;
//	}

	async fn run(&mut self, handler: &mut CommandHandler, args: String) -> Result<(), Error> {
		let stdin = handler.get_keystroke().await;
		println!("hi from rickroll: {:?}", stdin);
		Ok(())
	}
}
