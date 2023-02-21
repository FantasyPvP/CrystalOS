use async_std::task;
use async_trait::async_trait;
use rand::prelude::*;

mod crystal_rpg;

use crystal_rpg::{
    player::Player,
    entity::{Entity, Enemy, EntityObject},
    engine::{eventcheck, Choice, Event},
    renderer::{RENDERER, Element},
};

#[async_std::main]
async fn main() {
    let mut game = GameLoop::new();
    game.run(Vec::<String>::new()).await.unwrap();
}

#[derive(Debug)]
pub enum Error {
    Any(String),
}

#[async_trait]
pub trait Application {
	fn new() -> Self;

	async fn run(&mut self, args: Vec<String>) -> Result<(), Error> {
		Ok(())
	}
}


pub struct GameLoop;


#[async_trait]
impl Application for GameLoop {
    fn new() -> Self {
        Self {}
    }
    async fn run(&mut self, _args: Vec<String>) -> Result<(), Error> {

        let mut username: String = String::new();
        std::io::stdin().read_line(&mut username).unwrap();
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

        RENDERER.lock().unwrap().render_frame();
        let string = String::from(format!(
"┌──────────────────────────────────────────────────────────┐
│   {}                                                    
│   {} / {}                                              
└──────────────────────────────────────────────────────────┘"
        , player.username, player.health_points, player.max_health_points));
        let mut healthbar = Element::from_str(string);
        healthbar.render((1, 1));



        RENDERER.lock().unwrap().render_frame();
        Ok(())
    }
}

fn random() -> u64 {
    let mut r = crate::std::Random::random_int(0, 125);
    r
}















