use crate::std::application::Error;
use crate::std::application::Error::ApplicationError;
use crate::std::render::{ColouredChar, Dimensions, Frame, Position, RenderError, Window};
use crate::std::io::{Color, ColorCode, Display, KeyStroke, Stdin};
use crate::std::random::Random;
use crate::system::std::application::Application;
use crate::user::lib::libgui::cg_core::{CgComponent, Widget};
use crate::user::lib::libgui::cg_widgets::{CgContainer, CgLabel};
use crate::{serial_println, std};
use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use async_trait::async_trait;
use core::any::Any;

#[derive(Clone)]
pub struct Player {
    pub health: i32,
    pub position: Position<usize>,
}
impl Player {
    pub fn new() -> Player {
        Player {
            health: 5,
            position: Position::new(10, 12),
        }
    }
}

#[derive(Clone)]
pub struct Game {
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub score: u32,
    pub hit: bool,
    pub difficulty_idx: u8,
    pub gamespeed: f64,
    pub timer: GameTimer,
}

#[async_trait]
impl Application for Game {
    fn new() -> Self {
        Self {
            player: Player::new(),
            enemies: Vec::new(),
            score: 0,
            hit: false,
            difficulty_idx: 1,
            gamespeed: 1.0,
            timer: GameTimer::new(),
        }
    }
    async fn run(&mut self, window: Option<Window>, _args: Vec<String>) -> Result<(), Error> {
        let _d = Display::borrow();

        let mut container_data =
            CgContainer::new(Position::new(0, 0), Dimensions::new(80, 25), true);

        let score_label =
            Widget::insert(CgLabel::new(String::new(), Position::new(1, 1), 78, true));

        let self_ref = Widget::insert(self.clone());
        container_data.insert("app", self_ref);
        container_data.insert("score_label", score_label);
        let self_ref = container_data.fetch("app").unwrap();
        let score_ref = container_data.fetch("score_label").unwrap();

        loop {
            std::time::wait(0.01);

            if self.gameloop_iteration() {
                break;
            }

            self_ref.update(self.clone());
            score_ref.update(CgLabel::new(
                format!("< Score: {} >", self.score),
                Position::new(1, 1),
                78,
                true,
            ));

            if let Ok(frame) = container_data.render() {
                frame.write_to_screen().unwrap();
            }
            self.hit = false;

            // check if player has lost
            if self.player.health <= 0 {
                break;
            }
        }

        self.render_end_screen().await?;

        Ok(())
    }
}

impl Game {
    fn new_enemy(&mut self, speed: u8) {
        self.enemies.push(Enemy::new(Random::int(1, 21), speed));
    }

    fn check_difficulty(&mut self) {
        if self.difficulty_idx > 6 {
            self.new_enemy(2);
            self.new_enemy(1);
        } else if self.difficulty_idx > 4 {
            self.new_enemy(2);
        } else {
            self.new_enemy(1);
        }
    }

    // checks if an enemy should be spawned based on a random chance
    fn random_spawn_enemy(&mut self, chance: f64) {
        let mut randomdecimal = Random::int(0, 1000) as f64 / 1000.0;

        while randomdecimal > 1.0 {
            randomdecimal -= 1.0;
            self.check_difficulty();
        }

        // if the given chance is greater than the random number, spawn an enemy
        if chance > randomdecimal {
            self.check_difficulty();
        }
    }

    fn gameloop_iteration(&mut self) -> bool {
        // triggers roughly every 10ms

        match self.score {
            0..=9 => { self.gamespeed = 1.0;self.difficulty_idx = 1 },
            10..=24 => { self.gamespeed = 2.0;self.difficulty_idx = 2 },
            25..=49 => { self.gamespeed = 2.0;self.difficulty_idx = 3 },
            50..=99 => { self.gamespeed = 3.0;self.difficulty_idx = 4 } ,
            100..=199 => { self.gamespeed = 3.0;self.difficulty_idx = 5 },
            200..=500 => { self.gamespeed = 4.0;self.difficulty_idx = 6 },
            _ => { self.gamespeed = 5.0;self.difficulty_idx = 7 },
        };


        self.timer.advance();

        let game_update_delay = 5.0 / self.gamespeed;
        let _enemy_spawn_time = match self.difficulty_idx {
            1 => 10,
            2 => 10,
            3 => 10,
            4 => 5,
            5 => 2,
            _ => 1,
        };

        // check if enemies overlap with player, if so decrease player health and remove enemy
        self.enemies.retain(|e| {
            if e.position.1 == self.player.position.y as i16
                && (0..5)
                .map(|i| e.position.0 + i)
                .collect::<Vec<_>>()
                .contains(&(self.player.position.x as i16))
            {
                self.player.health -= 1;
                self.hit = true;
                false
            } else {
                true
            }
        });

        // check if a movement update is required
        if self.timer.get_move_time() as f64 >= game_update_delay {
            self.timer.reset_move_time();

            self.enemies.iter_mut().for_each(|e| {
                e.position.0 -= e.speed as i16;
            });

            // check for out of bounds enemies after move
            self.enemies.retain(|e| {
                if e.position.0 <= 0 {
                    self.score += 1;
                    false
                } else {
                    true
                }
            });
        }

        self.random_spawn_enemy(self.difficulty_idx as f64 / 10.0);

        if let Some(input_key) = Stdin::try_keystroke() {
            match input_key {
                KeyStroke::Char('`') => return true,
                KeyStroke::Char('w') => { if self.player.position.y > 0 { self.player.position.y -= 1 }},
                KeyStroke::Char('s') => { if self.player.position.y < 21 { self.player.position.y += 1 }},
                _ => (),
            }
        }

        false
    }

    async fn render_end_screen(&mut self) -> Result<(), Error> {
        let mut frame = Frame::new(Dimensions::new(0, 0), Dimensions::new(80, 25))
            .map_err(|_| ApplicationError("idk".to_string()))?;
        let msg = format!("your score was: {}", self.score);
        msg.chars().enumerate().for_each(|(i, c)| {
            serial_println!("{}", (80 - msg.len()) / 2 + i);
            frame[12][(80 - msg.len()) / 2 + i] = ColouredChar {
                character: c,
                colour: ColorCode::new(Color::Cyan, Color::Black),
            }
        });
        frame.write_to_screen().unwrap();

        while let KeyStroke::Char(c) = Stdin::keystroke().await {
            if c == '`' {
                break;
            }
        }

        Ok(())
    }
}

impl CgComponent for Game {
    fn render(&self) -> Result<Frame, RenderError> {
        let mut frame = Frame::new(Dimensions::new(1, 2), Dimensions::new(78, 22))?;

        let pos = self.player.position;

        let player_colour = match self.hit {
            true => Color::Red,
            false => Color::Cyan,
        };

        frame[pos.y][pos.x] = ColouredChar {
            character: '@',
            colour: ColorCode::new(player_colour, Color::Black),
        };

        for i in self
            .enemies
            .iter()
            .map(|enemy| enemy.position)
            .collect::<Vec<(i16, i16)>>()
        {
            frame[i.1 as usize][i.0 as usize] = ColouredChar {
                character: '«',
                colour: ColorCode::new(Color::LightGray, Color::Black),
            };
            (1..5).for_each(|offset| {
                if i.0 + offset < frame.dimensions.x as i16 {
                    frame[i.1 as usize][(i.0 + offset) as usize] = ColouredChar {
                        character: '═',
                        colour: ColorCode::new(Color::LightGray, Color::Black),
                    }
                }
            });
        }

        Ok(frame)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[derive(Clone)]
pub struct GameTimer {
    pub time_since_spawn: u32,
    pub time_since_move: u32,
}

impl GameTimer {
    pub fn new() -> GameTimer {
        GameTimer {
            time_since_spawn: 0,
            time_since_move: 0,
        }
    }

    pub fn advance(&mut self) {
        self.time_since_spawn += 1;
        self.time_since_move += 1;
    }
    
    pub fn get_move_time(&self) -> u32 {
        self.time_since_move
    }
    pub fn reset_move_time(&mut self) {
        self.time_since_move = 0
    }
}


#[derive(Clone)]
pub struct Enemy {
    pub position: (i16, i16), // x,y
    pub speed: u8,
}

impl Enemy {
    pub fn new(y: usize, speed: u8) -> Enemy {
        Enemy {
            position: (75, y as i16),
            speed: speed,
        }
    }
}
