use async_trait::async_trait;

use crate::std::application::{
	Application,
	Error,
};

use crate::{println};
use alloc::{string::String, boxed::Box, vec::Vec};


pub struct Rickroll {}

#[async_trait]
impl Application for Rickroll {
	fn new() -> Self {
		Self {}
	}

	async fn run(&mut self, _args: Vec<String>) -> Result<(), Error> {
		println!("

  _   _                        _____
 | \\ | |                      / ____|
 |  \\| | _____   _____ _ __  | |  __  ___  _ __  _ __   __ _
 | . ` |/ _ \\ \\ / / _ \\ '__| | | |_ |/ _ \\| '_ \\| '_ \\ / _` |
 | |\\  |  __/\\ V /  __/ |    | |__| | (_) | | | | | | | (_| |
 |_| \\_|\\___| \\_/ \\___|_|     \\_____|\\___/|_| |_|_| |_|\\__,_|
   _______            __     __          ___  ___
  / ____(_)           \\ \\   / /          | |  | |
 | |  __ ___   _____   \\ \\_/ /__  _   _  | |  | |_ __
 | | |_ | \\ \\ / / _ \\   \\   / _ \\| | | | | |  | | '_ \\
 | |__| | |\\ V /  __/    | | (_) | |_| | | |__| | |_) |
  \\_____|_| \\_/ \\___|    |_|\\___/ \\__,_|  \\____/| .__/
                                                | |
                                                |_|

");
		Ok(())
	}
}
