use async_trait::async_trait;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::interrupts;

use alloc::{string::{String, ToString}, vec::Vec, boxed::Box};

use crate::{
	kernel::render::{write, Color, RENDERER},
	kernel::tasks::keyboard::KEYBOARD,
	applications::*,
	std::application::{Error, Application},
	println,
	print, 
};












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

	let mut fetch = crystalfetch::CrystalFetch::new();
	let string = String::from(" ");
	let mut vec: Vec<String> = Vec::new();
	vec.push(string);
	fetch.run(vec).await.unwrap();

	CMD.lock().prompt();

	loop {
		let string = KEYBOARD.lock().get_string().await;
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
		Err(_) => { return Err(Error::EmptyCommand); }
	};

	match cmd.as_str() {
		"calculate"|"calc"|"solve" => {
			let mut cmd = calc::Calculator::new();
			cmd.run(args).await?;
		}

		"rickroll" => {
			let mut cmd = rickroll::Rickroll::new();
			cmd.run(args).await?;
		}

		"crystalfetch" => {
			let mut cmd = crystalfetch::CrystalFetch::new();
			cmd.run(args).await?;
		}
		"tasks" => {
			let mut cmd = tasks::Tasks::new();
			cmd.run(args).await?;
		}
		"play" => {
			let mut gameloop = crystal_rpg::init::GameLoop::new();
			gameloop.run(args).await?;
		}


		// direct OS functions (not applications)

		"echo" => { println!("Crystal: '{}'", args.into_iter().map(|mut s| { s.push_str(" "); s}).collect::<String>()) },

		"clear" => {
			interrupts::without_interrupts(|| {
					RENDERER.lock().clear();
			});
		}

		"print" => {
			use crate::kernel::os::OS;
			let x: String = OS.lock().version.clone();
			println!("{}", x);
		}
		"switch" => {
			if RENDERER.lock().sandbox == true {
				RENDERER.lock().text_mode().unwrap();
			} else {
				RENDERER.lock().sandbox_mode().unwrap();
			}
		}
		"random" => {
			use crate::std::random::Random;
			let vec = Vec::from(["a", "b", "c", "d", "e", "f"]);
			let sel = Random::selection(vec);
			println!("{}", sel);
		}

		_ => { return Err(Error::UnknownCommand("command not yet implemented".to_string())) },
	};

	Ok(())
}





pub struct CommandHandler {
	current: String,
	history: CmdHistory,
}

impl CommandHandler {
	pub fn new() -> Self {
		let handler = Self {
			current: String::new(),
			history: CmdHistory { history: Vec::new() },
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




