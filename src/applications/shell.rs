
use lazy_static::lazy_static;
use crate::{println, print, alloc::string::ToString};
use alloc::{string::String, vec::Vec};
use spin::Mutex;
use crate::applications::calc;

lazy_static! {
	pub static ref CMD: Mutex<CommandHandler> = Mutex::new(CommandHandler::new());
}

pub struct CommandHandler {
	current: String,
	history: CmdHistory,
}

impl CommandHandler {
	pub fn new() -> Self {
		Self {
			current: String::new(),
			history: CmdHistory { history: Vec::new() },
		}
	}
	
	pub fn from(string: String) -> Self {
		Self {
			current: string,
			history: CmdHistory { history: Vec::new() },
		}
	}
	
	pub fn input(&mut self, character: char) {
		print!("{}", &character);
		let (res, other): (char, bool) = match character {
			'\n' => (character, true),
			_ => (character, false)
		};
		self.current.push(res);

		if other {
			self.run();
		}
	}
	
	fn run(&mut self) -> Result<(), Error> {
		let mut command = false;

		println!("i ran a function! {}", &self.current);
		self.history.history.push(self.current.clone());
		let mut current = self.current.clone();
		current.pop();
		self.current = String::new();

		let (cmd, args) = match current.split_once(" ") {
			Some((x,y)) => { command = true; (x,y) },
			None => ("none", "none"),
		};

		if command == true {
			match cmd {
				"calculate"|"calc"|"solve" => { calc::calculate(args.to_string()); },
				"echo" => { println!("Crystal: '{}'", args) },
				_ => { println!("this command has not been implemented yet!"); },
			};
		} else {
			println!("this command does not exist! (or too few arguments supplied)")
		}
		print!(" >> ");
		
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

pub trait Application {
	fn stdin(string: String) -> Result<(), Error> { Ok(()) }
}
