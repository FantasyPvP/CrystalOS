use alloc::string::String;
use alloc::{format, vec, vec::Vec, boxed::Box};
use async_trait::async_trait;
use crate::std::io::{Color, Display, KeyStroke, Stdin};
use crate::std::time;
use crate::std::application::{Application, Error};
use crate::std::render::{ColorCode, ColouredChar, Dimensions, Frame, RenderError, Window};
use crate::std::random::Random;
use crate::system::std::render;
use super::super::super::lib::geometry::{Position, Direction};

#[derive(PartialEq)]
enum Gamemode {
    SinglePlayer,
    WithAI(u8, u8, u8), // number of ai, ai length, number of poi's
    Uninitialised,
}


#[derive(Clone, Debug, PartialEq)]
enum Status {
    Scored,
    Lost,
    Exited,
    None,
}

pub struct Game {
    snakes: Vec<Snake>,
    pois: Vec<Position>,
    score: u8,
    gamemode: Gamemode,
}



#[async_trait]
impl Application for Game {
    fn new() -> Self {
        Self {
            snakes: Vec::new(),
            pois: Vec::new(),
            score: 0,
            gamemode: Gamemode::Uninitialised,
        }
    }

    async fn run(&mut self, window: Option<Window>, args: Vec<String>) -> Result<(), Error> {

        let _settings = [0, 0, 0]; // ai_count, snake_len, poi_count

        if args.len() == 0 {
            self.gamemode = Gamemode::SinglePlayer;
        } else {
            match args[0].as_str() {
                "easy" => {
                    self.gamemode = Gamemode::SinglePlayer;
                },
                "normal" => {
                    self.gamemode = Gamemode::WithAI(5, 5, 5);
                },
                "impossible" => {
                    self.gamemode = Gamemode::WithAI(10, 15, 5);
                }
                "chaos" => {
                    self.gamemode = Gamemode::WithAI(20, 15, 20);
                }
                _ => {
                    self.gamemode = Gamemode::SinglePlayer;
                },
            }
        }

        // sets up game based on difficulty
        self.prepare();

        // switch OS to application mode
        let _d = Display::borrow();
        // render the initial state of the screen.
        self.render().map_err(|_| Error::ApplicationError(String::from("failed to render game screen")))?;
        // run the game
        self.gameloop().await?;
        Ok(())
    }
}

impl Game {
    fn prepare(&mut self) {
        self.snakes.push(Snake::player(0, 3));

        if let Gamemode::WithAI(ai_count, snake_len, poi_count) = self.gamemode {
            for i in 0..ai_count {
                self.snakes.push(Snake::ai(i as usize + 1, snake_len));
            }
            (0..poi_count).for_each(|_| self.new_poi());
        } else {
            self.new_poi()
        }
    }

    fn respawn_snakes(&mut self) {
        if let Gamemode::WithAI(_ai_count, snake_len, _poi) = self.gamemode {
            self.snakes.push(Snake::ai(self.snakes.len() + 1, snake_len));
        }
    }

    async fn gameloop(&mut self) -> Result<(), Error> { // main gameloop
        let mut _all_points: Vec<Position>;

        'gameloop: loop {

            time::wait(0.1);

            let mut _points: Vec<Position>;
            let length = self.snakes.len();

            for i in 0..length {
                let points: Vec<Position> = self.snakes.clone().into_iter().map(|s| s.tail).flatten().collect();
                let res = self.snakes[i].next(&points, &self.pois);

                match res {
                    Status::Lost => {
                        if self.snakes[i].ai_controlled {
                            self.snakes.remove(i);
                            self.respawn_snakes();
                        } else {
                            self.render_end_screen().map_err(|_| Error::ApplicationError(String::from("failed to render end screen")))?;
                            // loop triggers when game is lost
                            loop {
                                match Stdin::keystroke().await {
                                    KeyStroke::Char('x') => break 'gameloop,
                                    _ => continue,
                                }
                            }
                        }
                    },
                    Status::Exited => {
                        break 'gameloop
                    },
                    Status::Scored => {
                        if !self.snakes[i].ai_controlled {
                            self.score += 1;
                        }
                        self.replace_poi(&self.snakes[i].clone().head); // passes a reference to the location of the current snake's head
                    },
                    Status::None => {},
                }
            }
            self.snakes.retain(|s| s.alive);

            self.render().map_err(|_| Error::ApplicationError(String::from("failed to render game screen")))?;
        };
        Ok(())
    }

    fn new_poi(&mut self) {
        self.pois.push(Position { x: Random::int(3, 76) as i64, y: Random::int(3, 21) as i64 });
    }

    fn replace_poi(&mut self, poi: &Position) {
        self.pois.remove(self.pois.iter().position(|p| p == poi).unwrap());
        self.new_poi();
    }

    fn render(&mut self) -> Result<(), RenderError> {

        let mut frame = Frame::new(render::Position::new(0, 0), Dimensions::new(80, 25))?;
        let mut curr_colour = ColorCode::new(Color::LightBlue, Color::Black);

        for s in self.snakes.clone() {
            curr_colour = match s.ai_controlled {
                true => ColorCode::new(Color::Cyan, Color::Black),
                false => ColorCode::new(Color::LightGreen, Color::Black),
            };
            for point in s.tail.iter() {
                frame[24 - point.y as usize][point.x as usize] = ColouredChar::coloured('▓', curr_colour);
            }
        }

        self.pois.iter().for_each(|poi| {
            frame[24 - poi.y as usize][poi.x as usize] = ColouredChar::coloured('o', ColorCode::new(Color::Red, Color::Black));
        });

        let literal = format!("snake go brr score:  {}", self.score);
        let msg = Game::centre_text(80, literal);
        msg.chars().enumerate().for_each(|(i, c)| {
            if c != ' ' {
                frame[1][i] = ColouredChar::coloured(c, ColorCode::new(Color::LightGreen, Color::Black))
            }
        });

        frame.write_to_screen()?;

        Ok(())
    }


    fn centre_text(dims: usize, text: String) -> String { // centres text in a string of whitespace of a given length
        let max_pad = dims / 2;
        let mut msg = String::new();
        msg.push_str(" ".repeat(max_pad - round_up(text.len() as f64 / 2.0)).as_str());
        msg.push_str(text.as_str());
        msg.push_str(" ".repeat(max_pad - round_down(text.len() as f64 / 2.0 + 0.51)).as_str());
        msg
    }

    fn render_end_screen(&mut self) -> Result<(), RenderError> {
        let mut frame = Frame::new(render::Position::new(0, 0), Dimensions::new(80, 25))?;

        frame[10] = Game::centre_text(80, String::from("u lost")).chars().map(|c| ColouredChar::coloured(c, ColorCode::new(Color::Red, Color::Black))).collect();
        frame[12] = Game::centre_text(80, String::from(format!("ur score was {}", self.score))).chars().map(|c| ColouredChar::coloured(c, ColorCode::new(Color::LightGreen, Color::Black))).collect();
        frame[14] = Game::centre_text(80, String::from("L bozo")).chars().map(|c| ColouredChar::coloured(c, ColorCode::new(Color::Red, Color::Black))).collect();

        frame.write_to_screen()?;
        Ok(())
    }
}












#[derive(Debug, Clone)]
struct Snake {
    ai_controlled: bool,
    head: Position,
    tail: Vec<Position>,
    dir: Direction,
    alive: bool,
}

impl Snake {
    fn ai(id: usize, len: u8) -> Self {
        Self {
            ai_controlled: true,
            head: Position { x: 2 + 2*id as i64 * 2, y: 9 },
            tail: (1..=len as i64).map(|p| Position { x: 2 + 2*id as i64, y: 5 + p}).collect(),
            dir: Direction::Degrees0,
            alive: true,
        }
    }
    fn player(id: usize, len: u8) -> Self {
        Self {
            ai_controlled: false,
            head: Position { x: 2 + 2*id as i64, y: 9 },
            tail: (1..=len as i64).map(|p| Position { x: 2 + 2*id as i64, y: 5 + p}).collect(),
            dir: Direction::Degrees0,
            alive: true,
        }
    }

    fn next(&mut self, tails: &Vec<Position>, points_of_interest: &Vec<Position>) -> Status {  // returns (lose_condition, scored)

        // uses pathing algorithm if ai else keyboard input if human
        if self.ai_controlled {
            self.dir = PathFinder::decide(&self.head, tails, points_of_interest);
        } else {
            if let Some(KeyStroke::Char(c)) = Stdin::try_keystroke() {
                self.dir = match c {
                    'w' => Direction::Degrees0,
                    'a' => Direction::Degrees270,
                    's' => Direction::Degrees180,
                    'd' => Direction::Degrees90,
                    'x' => return Status::Exited,
                    _ => self.dir.clone(),
                };
            }
        }

        if self.dir != Direction::None {
            self.tail.push(self.head.clone());
        }

        match self.dir {
            Direction::Degrees0 => self.head.y += 1,
            Direction::Degrees180 => self.head.y -= 1,
            Direction::Degrees270 => self.head.x -= 1,
            Direction::Degrees90 => self.head.x += 1,
            Direction::None => {},
        }

        if self.lose_condition(tails) {
            self.alive = false;
            self.tail.remove(0);
            return Status::Lost;
        }

        if points_of_interest.contains(&self.head) {
            return Status::Scored;
        } else {
            if self.dir != Direction::None {
                self.tail.remove(0);
            }
        }

        Status::None
    }

    fn lose_condition(&mut self, tails: &Vec<Position>) -> bool { // where tails includes the tail of every other snake
        let p = self.head.clone();
        let snake_overlaps = tails.contains(&self.head); // checks if any part of the snake overlaps itself
        let out_of_bounds = p.x < 0 || p.y < 0 || p.x > 79 || p.y > 24; // checks if the snake goes out of bounds

        snake_overlaps || out_of_bounds
    }
}

struct PathFinder {}

impl PathFinder {
    fn decide(head: &Position, tails: &Vec<Position>, pois: &Vec<Position>) -> Direction {
        let nearest_poi = head.nearest(pois);
        let rel_pos = head.get_offset(&nearest_poi);

        // check actions don't lose them the game
        let mut possible_moves = Vec::new();
        let mut h: Position;

        h = Position { x: head.x + 1, y: head.y };
        if !(PathFinder::check_bounds(&h) || PathFinder::check_collision(&h, &tails)) {
            possible_moves.push(Direction::Degrees90);
        }
        h = Position { x: head.x - 1, y: head.y };
        if !(PathFinder::check_bounds(&h) || PathFinder::check_collision(&h, &tails)) {
            possible_moves.push(Direction::Degrees270);
        }
        h = Position { x: head.x, y: head.y + 1 };
        if !(PathFinder::check_bounds(&h) || PathFinder::check_collision(&h, &tails)) {
            possible_moves.push(Direction::Degrees0);
        }
        h = Position { x: head.x, y: head.y - 1 };
        if !(PathFinder::check_bounds(&h) || PathFinder::check_collision(&h, &tails)) {
            possible_moves.push(Direction::Degrees180);
        }

        if possible_moves.is_empty() {
            // panic!("no possible moves"); // use for debugging if a snake cannot find a move for some reason
            return Direction::Degrees90;
        } else {
            let optimal = PathFinder::optimal_move(head, &rel_pos, &possible_moves);
         //   serial_println!("{:?} {:?} {:?} {:?}", nearest_poi, rel_pos, head, optimal);
            return optimal;
        }
    }

    fn optimal_move(_head: &Position, rel_pos: &Position, moves: &Vec<Direction>) -> Direction {
        let mut optimal_moves = vec![Direction::None; 4];

        let x_offset: usize;
        let y_offset: usize;

        if rel_pos.x.abs() > rel_pos.y.abs() {
            y_offset = 1;
            x_offset = 0;
        } else {
            x_offset = 1;
            y_offset = 0;
        }

        if rel_pos.x < 0 {
            optimal_moves[x_offset] = Direction::Degrees270;
            optimal_moves[x_offset + 2] = Direction::Degrees90;
        } else {
            optimal_moves[x_offset] = Direction::Degrees90;
            optimal_moves[x_offset + 2] = Direction::Degrees270;
        }

        if rel_pos.y < 0 {
            optimal_moves[y_offset] = Direction::Degrees180;
            optimal_moves[y_offset + 2] = Direction::Degrees0;
        } else {
            optimal_moves[y_offset] = Direction::Degrees0;
            optimal_moves[y_offset + 2] = Direction::Degrees180;
        }
        //println!("moves: {:?}, optimal_moves: {:?}, rel_pos: {:?}", moves, optimal_moves, rel_pos);
        for m in optimal_moves {
            if moves.contains(&m) {
                return m;
            }
        };

        // this should never be used, the above statement should always return a value.
        panic!("No optimal move found (this should not happen)");
    }


    fn check_bounds(head: &Position) -> bool {
        head.x < 0 || head.y < 0 || head.x > 79 || head.y > 24
    }

    fn check_collision(head: &Position, tails: &Vec<Position>) -> bool {
        tails.contains(&head)
    }
}





fn round_up(n: f64) -> usize {
    (n + 0.99) as usize
}
fn round_down(n: f64) -> usize {
    n as usize
}

