use alloc::{string::String, vec::Vec, boxed::Box};
use async_trait::async_trait;

use crate::{std::{application::{Application, Error}, io::{Color, Display, KeyStroke, Stdin}, random::Random, render::{ColorCode, ColouredChar, Dimensions, Frame, Position, RenderError, Window}, time}, user::lib::libgui::cg_core::CgComponent};

pub struct Game {
    pub board: [[Cell; 7]; 6],
    pub turn: u8,
    pub vs_ai: bool,
    pub game_over: bool,
    pub winner: Option<u8>,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    Empty,
    Player1,
    Player2,
    Victory,
}

#[async_trait]
impl Application for Game {
    fn new() -> Self {
        Game {
            board: [[Cell::Empty; 7]; 6],
            turn: 1,
            vs_ai: false,
            game_over: false,
            winner: None,
        }
    }

    async fn run(&mut self, window: Option<Window>, _: Vec<String>) -> Result<(), Error> {
        let _display = Display::borrow();
        
        self.get_next_mode().await;

        // Main game loop
        loop {
            if self.game_over {
                self.render_end_screen().await.unwrap();
                let c = Stdin::keystroke().await; 
                match c {
                    KeyStroke::Char('`') => break,
                    _ => {
                        self.reset();
                        if !self.get_next_mode().await {
                            break;
                        }
                        self.game_over = false;
                    },
                }
                continue;
            }

            self.render().unwrap().write_to_screen().unwrap();

            if self.vs_ai && self.turn == 2 {
                // AI's turn
                self.make_ai_move();
                if !self.game_over {
                    self.turn = 1;
                }
                continue;
            }

            if let Some(key) = Stdin::last_keystroke() {
                match key {
                    KeyStroke::Char('`') => break,
                    KeyStroke::Char(c) => {
                        if let Some(col) = c.to_digit(10) {
                            if col > 0 && col <= 7 {
                                self.add_cell(col - 1, self.turn);
                                self.turn = if self.turn == 1 { 2 } else { 1 };
                            }
                        }
                    }
                    _ => {}
                }
            }

            self.apply_gravity();
            self.check_victory();
            time::wait(0.1);
        }

        Ok(())
    }
}

impl Game {
    fn apply_gravity(&mut self) {
        // Process each column
        for col in 0..7 {
            // Start from second-to-last row and move up
            // We don't need to check the bottom row since nothing can fall below it
            for row in (0..5).rev() {
                // If current cell is empty or the cell below is not empty, skip
                if let Cell::Empty = self.board[row][col] {
                    continue;
                }
                
                // Check if cell can fall one space
                if let Cell::Empty = self.board[row + 1][col] {
                    // Move cell down one space
                    self.board[row + 1][col] = self.board[row][col];
                    self.board[row][col] = Cell::Empty;
                }
            }
        }
    }

    async fn get_next_mode(&mut self) -> bool {
        // Game mode selection

        let mut frame = Frame::new(Position::new(0, 0), Dimensions::new(80, 25)).unwrap();
        let title = "Connect 4";
        let mode1 = "1. Player vs Player";
        let mode2 = "2. Player vs AI";
        
        // Center coordinates
        let center_y = 10;
        let title_x = 40 - (title.len() / 2) as u16;
        let mode1_x = 40 - (mode1.len() / 2) as u16;
        let mode2_x = 40 - (mode2.len() / 2) as u16;

        // Draw title
        for (i, c) in title.chars().enumerate() {
            frame[center_y][title_x as usize + i] = ColouredChar {
                character: c,
                colour: ColorCode::new(Color::Yellow, Color::Black),
            };
        }

        // Draw mode options
        for (i, c) in mode1.chars().enumerate() {
            frame[center_y + 2][mode1_x as usize + i] = ColouredChar {
                character: c,
                colour: ColorCode::new(Color::White, Color::Black),
            };
        }

        for (i, c) in mode2.chars().enumerate() {
            frame[center_y + 3][mode2_x as usize + i] = ColouredChar {
                character: c,
                colour: ColorCode::new(Color::White, Color::Black),
            };
        }

        frame.write_to_screen().unwrap();

        loop {        
            let key = Stdin::keystroke().await;
            match key {
                KeyStroke::Char('1') => {
                    self.vs_ai = false;
                    break true;
                }
                KeyStroke::Char('2') => {
                    self.vs_ai = true;
                    break true;
                }
                KeyStroke::Char('`') => break false,
                _ => {}
            }
        }
    }

    fn add_cell(&mut self, col: u32, player: u8) {
        if let Cell::Empty = self.board[0][col as usize] {
            self.board[0][col as usize] = match player {
                1 => Cell::Player1,
                2 => Cell::Player2,
                _ => panic!("Invalid player number"),
            };
        }
    }

    fn has_floating_tiles(&self) -> bool {
        for col in 0..7 {
            // Start from second-to-last row and move up
            for row in (0..5).rev() {
                // If current cell is not empty and cell below is empty, it's floating
                if self.board[row][col] != Cell::Empty && self.board[row + 1][col] == Cell::Empty {
                    return true;
                }
            }
        }
        false
    }

    fn check_victory(&mut self) {
        // Only check for victory if there are no floating tiles
        if self.has_floating_tiles() {
            return;
        }

        // Check horizontal
        for row in 0..6 {
            for col in 0..4 {
                let cell = self.board[row][col];
                if let Cell::Empty = cell { continue; }
                
                if (0..4).all(|i| self.board[row][col + i] == cell) {
                    // Mark winning cells
                    for i in 0..4 {
                        self.board[row][col + i] = Cell::Victory;
                    }
                    self.game_over = true;
                    self.winner = Some(if cell == Cell::Player1 { 1 } else { 2 });
                }
            }
        }

        // Check vertical
        for row in 0..3 {
            for col in 0..7 {
                let cell = self.board[row][col];
                if let Cell::Empty = cell { continue; }
                
                if (0..4).all(|i| self.board[row + i][col] == cell) {
                    // Mark winning cells
                    for i in 0..4 {
                        self.board[row + i][col] = Cell::Victory;
                    }
                    self.game_over = true;
                    self.winner = Some(if cell == Cell::Player1 { 1 } else { 2 });
                }
            }
        }

        // Check diagonal (down-right)
        for row in 0..3 {
            for col in 0..4 {
                let cell = self.board[row][col];
                if let Cell::Empty = cell { continue; }
                
                if (0..4).all(|i| self.board[row + i][col + i] == cell) {
                    // Mark winning cells
                    for i in 0..4 {
                        self.board[row + i][col + i] = Cell::Victory;
                    }
                    self.game_over = true;
                    self.winner = Some(if cell == Cell::Player1 { 1 } else { 2 });
                }
            }
        }

        // Check diagonal (down-left)
        for row in 0..3 {
            for col in 3..7 {
                let cell = self.board[row][col];
                if let Cell::Empty = cell { continue; }
                
                if (0..4).all(|i| self.board[row + i][col - i] == cell) {
                    // Mark winning cells
                    for i in 0..4 {
                        self.board[row + i][col - i] = Cell::Victory;
                    }
                    self.game_over = true;
                    self.winner = Some(if cell == Cell::Player1 { 1 } else { 2 });
                }
            }
        }

        // Check for draw
        if !self.game_over {
            if (0..6).all(|row| (0..7).all(|col| self.board[row][col] != Cell::Empty)) {
                self.game_over = true;
                self.winner = None;
            }
        }
    }

    fn make_ai_move(&mut self) {
        loop {
            let col = (Random::int(0, 6)) as u32;
            // Check if column is not full
            if self.board[0][col as usize] == Cell::Empty {
                self.add_cell(col, 2);
                break;
            }
        }
    }

    fn reset(&mut self) {
        self.board = [[Cell::Empty; 7]; 6];
        self.turn = 1;
        self.game_over = false;
        self.winner = None;
    }

    async fn render_end_screen(&mut self) -> Result<(), RenderError> {
        let mut frame = Frame::new(Position::new(0, 0), Dimensions::new(80, 25))?;
        
        // Game Over message
        let game_over = "Game Over!";
        let winner_text = match self.winner {
            Some(1) => "Player 1 Wins!",
            Some(2) => if self.vs_ai { "AI Wins!" } else { "Player 2 Wins!" },
            None => "Draw!",
            _ => panic!("this shouldn't be possible"),
        };
        let restart_text = "Press any key to play again";
        
        let center_y = 12;
        let game_over_x = 40 - (game_over.len() / 2) as u16;
        let winner_x = 40 - (winner_text.len() / 2) as u16;
        let restart_x = 40 - (restart_text.len() / 2) as u16;

        // Draw game over
        for (i, c) in game_over.chars().enumerate() {
            frame[center_y][game_over_x as usize + i] = ColouredChar {
                character: c,
                colour: ColorCode::new(Color::White, Color::Black),
            };
        }

        // Draw winner
        for (i, c) in winner_text.chars().enumerate() {
            frame[center_y + 1][winner_x as usize + i] = ColouredChar {
                character: c,
                colour: ColorCode::new(Color::Yellow, Color::Black),
            };
        }

        // Draw restart instruction
        for (i, c) in restart_text.chars().enumerate() {
            frame[center_y + 2][restart_x as usize + i] = ColouredChar {
                character: c,
                colour: ColorCode::new(Color::White, Color::Black),
            };
        }

        frame.write_to_screen()?;
        Ok(())
    }

    fn render(&self) -> Result<Frame, RenderError> {
        let mut frame = Frame::new(Position::new(0, 0), Dimensions::new(80, 25))?;

        // Calculate center position to align board
        let cell_width = 4;  // 3 for cell + 1 for separator
        let cell_height = 3; // 2 for cell + 1 for gap
        let board_width = 7 * cell_width - 1;  // -1 because last separator not needed
        let board_height = 6 * cell_height - 1; // -1 because last gap not needed
        let start_x = (80 - board_width) / 2;
        let start_y = (25 - board_height) / 2;

        // Draw column numbers
        for col in 0..7 {
            let x = start_x + (col * cell_width);
            // Center the 3-char number display "[X]" in the 3-space cell width
            frame[start_y - 3][x] = ColouredChar::coloured('[', ColorCode::new(Color::White, Color::Black));
            frame[start_y - 3][x + 1] = ColouredChar::coloured((1 + col as u8 + b'0') as char, ColorCode::new(Color::White, Color::Black));
            frame[start_y - 3][x + 2] = ColouredChar::coloured(']', ColorCode::new(Color::White, Color::Black));
        }

        // Draw each cell
        for row in 0..6 {
            let y = start_y + (row * cell_height);

            for col in 0..7 {
                let x = start_x + (col * cell_width);

                // Draw vertical separator after each cell (except last column)
                if col < 6 {
                    let separator_x = x + 3;
                    for dy in 0..3 {
                        frame[y -1 + dy][separator_x] = ColouredChar::coloured('│', ColorCode::new(Color::White, Color::Black));
                    }
                }

                // Set color based on cell state
                let color = match self.board[row][col] {
                    Cell::Empty => continue,
                    Cell::Player1 => ColorCode::new(Color::Red, Color::Red),
                    Cell::Player2 => ColorCode::new(Color::Yellow, Color::Yellow),
                    Cell::Victory => ColorCode::new(Color::Green, Color::Green),
                };

                // Draw a 3x2 block for each cell
                for dy in 0..2 {
                    for dx in 0..3 {
                        frame[y + dy][x + dx] = ColouredChar::coloured('█', color);
                    }
                }

            }

            // Draw horizontal gap after each row (except last row)
            if row < 5 {
                let gap_y = y + 2;
                for x in start_x..start_x + board_width {
                    frame[gap_y][x] = ColouredChar::coloured(' ', ColorCode::new(Color::White, Color::Black));
                }
            }
        }

        Ok(frame)
    }
    
    fn as_any(&self) -> &dyn core::any::Any {
        todo!()
    }
}