use alloc::{string::String, vec::Vec, boxed::Box};

use crate::std::{application::{Application, Error}, render::Window};

use async_trait::async_trait;

pub struct ZxqSH {
    history: Vec<String>,
    idx: u32,

    window: Window,

}

#[async_trait]
impl Application for ZxqSH {
    fn new() -> Self {
        todo!()
    }

    async fn run(&mut self, window: Option<Window>, _args: Vec<String>) -> Result<(), Error> {        
        loop {
            if let Ok(exit) = self.next().await {
                if exit { return Ok(()) }
            }
        }
    }
}

impl ZxqSH {
    async fn next(&mut self) -> Result<bool, Error> {

        // update cycle for the shell

        // TOOD: prompt

        // TODO: exit if necessar

        // TODO: execute command

        // return

        Ok(false)
    }


    async fn execute(&self, command: String) -> Result<(), Error> {
        Ok(())
    }
}