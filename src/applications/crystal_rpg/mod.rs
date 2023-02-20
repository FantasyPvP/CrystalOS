use lazy_static::lazy_static;
use async_trait::async_trait;
use crate::shell::{
	Application,
	Error,
};
use crate::os::OS;
use crate::{println};
use alloc::{boxed::Box, string::String, vec::Vec};
use crate::vga_buffer::{Color, write};



pub struct GameLoop;


#[async_trait]
impl Application for GameLoop {
    fn new() -> Self {
        Self {}
    }
    async fn run(&mut self, _args: Vec<String>) -> Result<(), Error> {
        
    }
}










pub struct Player {
    health_points: u32,
    max_health_points: u32,
    base_attack_damage: u32,

    inventory: [ Item ; 15 ],
    equipped: [ Item ; 7 ], // helmet, chestplate, leggings, boots, mainhand, offhand, charm
}

pub enum Item {

}