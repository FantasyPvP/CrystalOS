use async_trait::async_trait;
use alloc::{string::String, vec::Vec, boxed::Box};

#[async_trait]
pub trait Application {
	fn new() -> Self;

	async fn run(&mut self, _: Vec<String>) -> Result<(), Error> {
		Ok(())
	}
}

#[derive(Debug)]
pub enum Error {
	UnknownCommand(String),
	CommandFailed(String),
	EmptyCommand,
}