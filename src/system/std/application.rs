use async_trait::async_trait;
use alloc::{string::String, vec::Vec, boxed::Box};

use super::render::Window;

#[async_trait]
pub trait Application {
	fn new() -> Self;

	async fn run(&mut self, _window: Option<Window>, _args: Vec<String>) -> Result<(), Error> {
		Ok(())
	}
}

#[derive(Debug)]
pub enum Error {
	UnknownCommand(String),
	CommandFailed(String),
	ApplicationError(String),
	EmptyCommand,
}

pub enum Exit {
	None,
	Exit,
	ExitWithError(Error),
}