
use async_trait::async_trait;
use lazy_static::lazy_static;
use crate::{println, print, alloc::string::ToString};
use alloc::{string::String, vec::Vec, boxed::Box};
use spin::Mutex;
use crate::applications::{
	calc::Calculator,
	rickroll::Rickroll,
	crystalfetch::CrystalFetch,
	tasks::Tasks,
};
use crate::tasks::keyboard::ScanCodeStream;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use futures_util::stream::StreamExt;
use crate::render::{write, Color, RENDERER};
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
	let string = String::from(" ");
	let mut vec: Vec<String> = Vec::new();
	vec.push(string);
	fetch.run(vec).await;

	CMD.lock().prompt();

	loop {
		let string = CMD.lock().get_string().await;
		CMD.lock().current.push_str(&string);
		match exec().await {
			Ok(_) => { (); },
			Err(e) => { handle_error(e); },
		};
		CMD.lock().prompt();
	}
}

fn handle_error(e: Error) {

}


async fn exec() -> Result<(), Error> {
	let mut current = CMD.lock().current.clone();

	CMD.lock().history.history.push(current.clone());

	current.pop();
	CMD.lock().current = String::new();

	let (cmd, args) = match CommandHandler::parse_args(current) {
		Ok((cmd, args)) => { (cmd, args) },
		Err(x) => { return Err(Error::EmptyCommand); }
	};
	match cmd.as_str() {
		"calculate"|"calc"|"solve" => {
			let mut cmd = Calculator::new();
			cmd.run(args).await?;
		}

		"rickroll" => {
			let mut cmd = Rickroll::new();
			cmd.run(args).await?;
		}

		"crystalfetch" => {
			let mut cmd = CrystalFetch::new();
			cmd.run(args).await?;
		}
			"tasks" => {
			let mut cmd = Tasks::new();
			cmd.run(args).await?;
		}
		// direct OS functions (not applications)

		"echo" => { println!("Crystal: '{}'", args.into_iter().map(|mut s| { s.push_str(" "); s}).collect::<String>()) },

		"clear" => {
			interrupts::without_interrupts(|| {
					RENDERER.lock().clear();
			});
		}

		"print" => {
			use crate::os::OS;
			let x: String = OS.lock().version.clone();
			println!("{}", x);
		}
		"switch" => {
			use crate::render::RENDERER;
			if RENDERER.lock().sandbox == true {
				RENDERER.lock().text_mode().unwrap();
			} else {
				RENDERER.lock().sandbox_mode().unwrap();
			}
		}
		"random" => {
			use crate::std::Random;
			let vec = Vec::from(["a", "b", "c", "d", "e", "f"]);
			let sel = Random::selection(vec);
			println!("{}", sel);
		}

		_ => { return Err(Error::UnknownCommand("command not yet implemented".to_string())) },
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

	pub fn parse_args(command: String) -> Result<(String, Vec<String>), String> {

		let temp = command.split(" ").collect::<Vec<&str>>();
		let mut args: Vec<String> = Vec::new();
		for arg in temp {
			match arg {
				"" => {},
				x => args.push(x.to_string())
			}
		};
		let cmd: String;
		if args.len() > 0 {
			cmd = args[0].clone();
			args.remove(0);
		} else {
			return Err("command was empty.".to_string());
		};
		Ok((cmd, args))
	}

	// this function is activated every time the user presses a key on the keyboard
	// it accesses the queue of keys (a static ref in src/tasks/keyboard.rs)

	pub async fn get_keystroke_inner(&mut self) -> Option<char> {
		loop {
			if let Some(scancode) = self.scancodes.next().await {
				if let Ok(Some(key_event)) = self.keyboard.add_byte(scancode) {
					if let Some(key) = self.keyboard.process_keyevent(key_event) {
						match key {
							DecodedKey::Unicode(character) => {
								if character == b'\x08' as char { // checks if the character is a backspace
									interrupts::without_interrupts(|| {
										RENDERER.lock().backspace(); // runs the backspace function of the vga buffer to remove the last character
									});
									return None;
								} else {
									return Some(character);
								}
							},
							DecodedKey::RawKey(key) => { print!("{:?}", key) },
						}
					}
				}
			}
		}
	}

	pub async fn get_keystroke(&mut self) -> char {
		loop {
			match self.get_keystroke_inner().await {
				Some(c) => return c,
				None => ()
			}
		}
	}


	pub async fn get_string(&mut self) -> String {
		let mut val = String::new();
		loop {
			let character = match self.get_keystroke_inner().await {
				Some(c) => { c },
				None => { val.pop(); continue; },
			};
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
	EmptyCommand,
}

#[async_trait]
pub trait Application {
	fn new() -> Self;

	async fn run(&mut self, args: Vec<String>) -> Result<(), Error> {
		Ok(())
	}
}
