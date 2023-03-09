use async_trait::async_trait;
use rand::prelude::*;

use super::{
    engine::{eventcheck, Choice, Event},
    entity::{Entity, Enemy, EntityObject},
    player::Player,
};

use alloc::{boxed::Box, string::{String, ToString}, vec::Vec, format, borrow::ToOwned};

use crate::{ 
    std::application::{
	    Application,
	    Error,
    },
    std::{
        io::{self, println, serial_println, FRAMEGEN, Element},
        random, 
    },
};


pub struct GameLoop;


#[async_trait]
impl Application for GameLoop {
    fn new() -> Self {
        Self {}
    }
    async fn run(&mut self, _args: Vec<String>) -> Result<(), Error> {

        let mut username: String = io::stdin().await;
        username = username.trim().to_string();

        let mut player = Player::new(username);

        let mut enemy = Enemy::new();

        for _ in 0..30 {
            match (eventcheck(player.attack_entity(&mut EntityObject::Enemy(&mut enemy)))) {
                Choice::A(result) => {
                    println!("{}", result);
                },
                Choice::B(event) => {
                    println!("{}", event);
                    match event {
                        Event::PlayerKilled => {
                            println!(" [!] {} was slain by Enemy\n\n[ You lost! ]", player.username);
                            break;
                        }
                        Event::EntityKilled(entity) => {
                            println!("\n [!] Enemy was slain by {}\n\n [ You won! ]", player.username);
                            break;
                        }
                    }
                }
            }
            println!("{}", eventcheck(enemy.attack_entity(&mut EntityObject::Player(&mut player))));
            println!("[{}\n[{}", player, enemy);
        }
 
        FRAMEGEN.lock().render_frame();

       
        let string = String::from(format!(
"┌────────────────────────────┐
│   {}                        
│   {} / {}                     
└────────────────────────────┘"
        , player.username, player.health_points, player.max_health_points));
        let mut healthbar = Element::from_str(string);
        healthbar.render((1, 1));

        let new2 = String::from("slushy stfu");
        let mut new = Element::from_str(new2);

        new.render((10, 20));



        FRAMEGEN.lock().render_frame();

        let fr = FRAMEGEN.lock().get_frame().to_owned();
        serial_println!("{}", {
            let mut string = String::new();
            for row in fr {
                let mut r = String::new();
                for col in row {
                    r.push(col);
                }
                string.push_str(&r);
                string.push('\n')
            };
            string
        });


        loop {
            println!("{}", io::stdchar().await)
        }

        Ok(())
    }
}

fn random() -> u64 {
    let mut r = random::Random::int(0, 125) as u64;
    r
}















