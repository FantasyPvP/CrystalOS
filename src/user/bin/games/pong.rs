use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::any::Any;
use async_trait::async_trait;
use crate::std::application::{Application, Error};
use crate::std::render::{ColorCode, ColouredChar, Dimensions, Frame, Position, RenderError, Window, BUFFER_HEIGHT, BUFFER_WIDTH};
use crate::std::io::{Color, Display, KeyStroke, Stdin};
use crate::std::time::Timer;
use crate::user::lib::libgui::cg_core::CgComponent;

pub(crate) struct Game {
    ball: Ball,
    player1: Player,
    player2: Player,
}

#[async_trait]
impl Application for Game {
    fn new() -> Self {
        Game {
            ball: Ball::new(),
            player1: Player::new(1),
            player2: Player::new(78),
        }
    }

    async fn run(&mut self, window: Option<Window>, _: Vec<String>) -> Result<(), Error> {
        let _d = Display::borrow();

        let mut update_time = Timer::new(0.1);

        let mut updated;
        loop {
            updated = false;

            if let Some(key) = Stdin::try_keystroke() {
                match key {
                    KeyStroke::Char('w') => {
                        self.player1.move_player(-1);
                        updated = true;
                    },
                    KeyStroke::Char('s') => {
                        self.player1.move_player(1);
                        updated = true;
                    },
                    KeyStroke::Up => {
                        self.player2.move_player(-1);
                        updated = true;
                    },
                    KeyStroke::Down => {
                        self.player2.move_player(1);
                        updated = true;
                    },
                    KeyStroke::Char('`') => break,
                    _ => {}
                }
            }
            if update_time.is_done() {
                updated = true;
                self.update_ball();
                update_time.reset()
            }

            if updated {
                if let Ok(frame) = self.render() {
                    frame.write_to_screen().unwrap();
                }
            }
        }
        Ok(())
    }
}

impl Game {
    fn update_ball(&mut self) {
        let pos_next_f32 = Position::new( // invert x direction on collision with player
                                          self.ball.pos.x + self.ball.vx,
                                          self.ball.pos.y + self.ball.vy,
        );

        if pos_next_f32.y < 0.0 || pos_next_f32.y >= BUFFER_HEIGHT as f32 { // if the move is outside the screen, then invert the direction
            self.ball.vy = -self.ball.vy;
        }

        if pos_next_f32.x < 0.0 {
            self.player2.score += 1;
            self.ball.pos = Position::new(40.0, 12.0);
            self.ball.vx = 1.0;
            self.ball.vy = 0.3;
        }

        if pos_next_f32.x >= BUFFER_WIDTH as f32 {
            self.player1.score += 1;
            self.ball.pos = Position::new(40.0, 12.0);
            self.ball.vx = -1.0;
            self.ball.vy = 0.3;
        }

        let pos_next = Position::new(
            pos_next_f32.x as usize,
            pos_next_f32.y as usize,
        );

        for i in 0..5 {
            if self.player1.pos.y + i - 2 == pos_next.y && self.player1.pos.x == pos_next.x {
                self.ball.vx = -self.ball.vx;
                break;
            } else if self.player2.pos.y + i - 2 == pos_next.y && self.player2.pos.x == pos_next.x {
                self.ball.vx = -self.ball.vx;
                break;
            }
        };

        self.ball.pos = Position::new(
            self.ball.pos.x + self.ball.vx,
            self.ball.pos.y + self.ball.vy
        )
    }
}

impl CgComponent for Game {
    fn render(&self) -> Result<Frame, RenderError> {
        let mut frame = Frame::new(Dimensions::new(0, 0), Dimensions::new(80, 25))?;

        for y in 0..5 {
            frame.write(Position::new(self.player1.pos.x, self.player1.pos.y + y -2), ColouredChar::coloured('▓', ColorCode::new(Color::Cyan, Color::Black))).unwrap();
            frame.write(Position::new(self.player2.pos.x, self.player2.pos.y + y -2), ColouredChar::coloured('▓', ColorCode::new(Color::Cyan, Color::Black))).unwrap();
        }
        frame.write(self.ball.pos.into_usize().unwrap(), ColouredChar::coloured('O', ColorCode::new(Color::Green, Color::Black))).unwrap();

        Ok(frame)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct Player {
    pos: Position<usize>,
    score: i32,
}


impl Player {
    fn new(x: usize) -> Self {
        Player { pos: Position::new(x, 12), score: 0 }
    }

    // valid for |y| = 1
    fn move_player(&mut self, y: i32) {
        if self.pos.y < 3 && y < 0 {
            return;
        } else if self.pos.y >= BUFFER_HEIGHT - 3 && y > 0 {
            return;
        }
        self.pos.y = (self.pos.y as i32 + y) as usize;
    }
}

struct Ball {
    pos: Position<f32>,
    vx: f32,
    vy: f32,
}

impl Ball {
    fn new() -> Self {
        Ball {  pos: Position::new(40.0, 12.0), vx: 1.0, vy: 0.3 }
    }
}
