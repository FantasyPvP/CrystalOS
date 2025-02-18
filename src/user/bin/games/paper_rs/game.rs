use core::any::Any;

use alloc::{boxed::Box, format, string::String, vec::Vec};
use async_trait::async_trait;

use crate::{
    std::{
        self, 
        application::{Application, Error}, 
        io::{Color, ColorCode, Display, KeyStroke, Stdin}, 
        render::{ColouredChar, Dimensions, Frame, Position, RenderError, Window}, 
        time
    }, 
    user::lib::libgui::cg_core::CgComponent
};

use super::player::Player;

#[derive(Copy, Clone)]
pub enum Cell {
    Empty,
    Solid(u8, bool),
    Tail(u8, bool),
    Head(u8, bool),
}

pub struct GameBoard {
    board: [[Cell; 80]; 25],
    players: [Player; 6],
    max_territory: u32,  // Track maximum territory captured
    current_territory: u32,  // Track current territory
}

#[async_trait]
impl Application for GameBoard {
    fn new() -> GameBoard {
        GameBoard {
            board: [[Cell::Empty; 80]; 25],
            players: [
                Player::new(0, (10, 10), false),
                Player::new(1, (70, 10), true),
                Player::new(2, (10, 15), true),
                Player::new(3, (70, 15), true),
                Player::new(4, (35, 5), true),
                Player::new(5, (35, 20), true),
            ],
            max_territory: 0,
            current_territory: 0,
        }
    }

    async fn run(&mut self, window: Option<Window>, _args: Vec<String>) -> Result<(), Error> {
        let _display = Display::borrow();

        'outer: loop {
            // Set initial positions as solid territory
            for player in &self.players {
                let (x, y) = player.position;
                self.board[y as usize][x as usize] = Cell::Head(player.id, true);
            }
    
            let mut player_direction: (i32, i32) = (0, 0);
    
            // player controls player 1.
            loop {
                self.render().unwrap().write_to_screen().unwrap();
                time::wait(0.1);
    
                // first get player input
                if let Some(keystroke) = Stdin::try_keystroke() {
                    match keystroke {
                        KeyStroke::Up => player_direction = (0, -1),
                        KeyStroke::Down => player_direction = (0, 1),
                        KeyStroke::Left => player_direction = (-1, 0),
                        KeyStroke::Right => player_direction = (1, 0),
                        KeyStroke::Char('w') | KeyStroke::Char('W') => player_direction = (0, -1),
                        KeyStroke::Char('s') | KeyStroke::Char('S') => player_direction = (0, 1),
                        KeyStroke::Char('a') | KeyStroke::Char('A') => player_direction = (-1, 0),
                        KeyStroke::Char('d') | KeyStroke::Char('D') => player_direction = (1, 0),
                        KeyStroke::Char('`') => break 'outer,
                        _ => {},
                    }
                }
                
                // perform movement commands
                self.move_player(0, player_direction.0, player_direction.1);
    
                self.update_ai_players();
    
                self.run_fill_algorithm();
    
    
                if !self.players[0].alive {
                    self.render_end_screen().await.unwrap();
                    break;
                }
            }
    
            // Wait for any key to exit
            match Stdin::keystroke().await {
                KeyStroke::Char('`') => break 'outer,
                _ => self.reset(),
            }
        }
        
        Ok(())
    }
}


impl CgComponent for GameBoard {
    fn render(&self) -> Result<Frame, RenderError> {
        let mut frame = Frame::new(Position::new(0, 0), Dimensions::new(80, 25))?;
        
        for i in 0..25 {
            for j in 0..80 {

                let character: char;
                let pid: Option<u8>;

                match self.board[i][j] {
                    Cell::Empty => { character = ' '; pid = None; }
                    Cell::Solid(p, _) => { character = '█'; pid = Some(p); }
                    Cell::Tail(p, _) => { character = '▒'; pid = Some(p); }
                    Cell::Head(p, _) => { character = '▓'; pid = Some(p); } 
                }

                let colour = if let Some(p) = pid {
                    match p {
                        0 => ColorCode::new(Color::Green, Color::Black),
                        1 => ColorCode::new(Color::Blue, Color::Black),
                        2 => ColorCode::new(Color::Red, Color::Black),
                        3 => ColorCode::new(Color::Yellow, Color::Black),
                        4 => ColorCode::new(Color::Magenta, Color::Black),
                        _ => ColorCode::new(Color::Cyan, Color::Black),
                    }
                } else {
                    ColorCode::new(Color::White, Color::Black)
                };

                frame[i][j] = ColouredChar {
                    character,
                    colour,
                };
            }
        }

        Ok(frame)
    }
    
    fn as_any(&self) -> &dyn core::any::Any {
        self as &dyn Any 
    }
}

impl GameBoard {
    const UP: (i32, i32) = (0, -1);
    const DOWN: (i32, i32) = (0, 1);
    const LEFT: (i32, i32) = (-1, 0);
    const RIGHT: (i32, i32) = (1, 0);
    const DIRS: [(i32, i32); 8] = [
        Self::UP, Self::DOWN, Self::LEFT, Self::RIGHT,
        (1, 1), (1, -1), (-1, 1), (-1, -1),
    ];


    fn reset(&mut self) {
        self.board = [[Cell::Empty; 80]; 25];
        self.players = [
            Player::new(0, (10, 10), false),
            Player::new(1, (70, 10), true),
            Player::new(2, (10, 15), true),
            Player::new(3, (70, 15), true),
            Player::new(4, (35, 5), true),
            Player::new(5, (35, 20), true),
        ];

        // Set initial positions as solid territory
        for player in &self.players {
            let (x, y) = player.position;
            self.board[y as usize][x as usize] = Cell::Head(player.id, true);
        }

        self.max_territory = 0;
        self.current_territory = 0;
    }

    fn respawn_ai_player(&mut self, player_id: u8) {
        // Try up to 10 times to find a valid spawn position
        for _ in 0..10 {
            let x = std::random::Random::int(5, 75) as i32;
            let y = std::random::Random::int(5, 20) as i32;
            
            // Check if position is empty or not in someone's territory
            if let Cell::Empty = self.board[y as usize][x as usize] {
                let dir_idx = std::random::Random::int(0, 3) as usize;
                self.players[player_id as usize] = Player::new(player_id, (x, y), true);
                // Place the head at spawn position
                self.board[y as usize][x as usize] = Cell::Head(player_id, true);
                return;
            }
        }
        
        // If we couldn't find a spot after 10 tries, just keep the player dead
        self.players[player_id as usize].alive = false;
    }

    fn count_territory(&self) -> u32 {
        let mut count = 0;
        for row in &self.board {
            for cell in row {
                if let Cell::Solid(0, true) = cell {
                    count += 1;
                }
            }
        }
        count
    }

    async fn render_end_screen(&mut self) -> Result<(), RenderError> {
        let mut frame = self.render()?;
        
        // Game Over message
        let game_over_text = "Game Over!";
        let center_y = 12;
        let center_x = 40 - (game_over_text.len() / 2) as u16;

        // Draw the game over text
        for (i, c) in game_over_text.chars().enumerate() {
            let pos = center_x + i as u16;
            frame[center_y][pos as usize] = ColouredChar {
                character: c,
                colour: ColorCode::new(Color::White, Color::Black),
            };
        }

        // Show player's score
        let score_text = format!("Score: {}", self.current_territory);
        let score_x = 40 - (score_text.len() / 2) as u16;
        for (i, c) in score_text.chars().enumerate() {
            let pos = score_x + i as u16;
            frame[center_y + 1][pos as usize] = ColouredChar {
                character: c,
                colour: ColorCode::new(Color::White, Color::Black),
            };
        }

        // Show restart instruction
        let restart_text = "Press any key to restart";
        let restart_x = 40 - (restart_text.len() / 2) as u16;
        for (i, c) in restart_text.chars().enumerate() {
            let pos = restart_x + i as u16;
            frame[center_y + 2][pos as usize] = ColouredChar {
                character: c,
                colour: ColorCode::new(Color::White, Color::Black),
            };
        }

        frame.write_to_screen()?;

        Ok(())
    }

    fn move_player(&mut self, playerid: u8, dx: i32, dy: i32) {
        if let Some(eliminated_id) = self.players[playerid as usize].move_player(dx, dy, &mut self.board) {
            self.players[eliminated_id as usize].alive = false;
            // Clear territory and respawn if AI
            if eliminated_id != 0 {
                self.clear_player_territory(eliminated_id);
                self.respawn_ai_player(eliminated_id);
            } else {
                // Update max territory before game over
                self.current_territory = self.count_territory();
                self.max_territory = self.max_territory.max(self.current_territory);
            }
        }
    }

    fn update_ai_players(&mut self) {
        // Skip player 0 (human player)
        for i in 1..6 {
            let dir = self.players[i].update_ai(&self.board);
            if dir != (0, 0) {
                self.move_player(i as u8, dir.0, dir.1);
            }
        }
    }

    fn run_fill_algorithm(&mut self) {
        // Pre-allocate a single reusable grid
        let mut fill_grid = [[false; 80]; 25];
        let mut queue = Vec::with_capacity(80 * 25);
        
        // Process for each alive player
        for player in self.players.iter().filter(|p| p.alive) {
            let pid = player.id;
            
            // Check if player's head is touching their territory
            let (px, py) = player.position;
            let mut head_touching_territory = false;
            
            // Check all adjacent cells to head
            for (dy, dx) in Self::DIRS.iter().take(4) { // Only check cardinal directions
                let ny = py + dy;
                let nx = px + dx;
                
                if ny >= 0 && ny < 25 && nx >= 0 && nx < 80 {
                    if let Cell::Solid(p, _) = self.board[ny as usize][nx as usize] {
                        if p == pid {
                            head_touching_territory = true;
                            break;
                        }
                    }
                }
            }
            
            // Skip if head is not touching territory
            if !head_touching_territory {
                continue;
            }
            
            // Reset grid
            for row in fill_grid.iter_mut() {
                row.fill(false);
            }
            
            // Mark all player lines as walls but don't fill them yet
            for y in 0..25 {
                for x in 0..80 {
                    if let Cell::Head(p, _) | Cell::Tail(p, _) | Cell::Solid(p, _) = self.board[y][x] {
                        if p == pid {
                            fill_grid[y][x] = true;
                        }
                    }
                }
            }
            
            // Clear queue for reuse
            queue.clear();
            
            // Add edge cells to queue
            for x in 0..80 {
                if !fill_grid[0][x] {
                    queue.push((0, x));
                    fill_grid[0][x] = true;
                }
                if !fill_grid[24][x] {
                    queue.push((24, x));
                    fill_grid[24][x] = true;
                }
            }
            for y in 1..24 {
                if !fill_grid[y][0] {
                    queue.push((y, 0));
                    fill_grid[y][0] = true;
                }
                if !fill_grid[y][79] {
                    queue.push((y, 79));
                    fill_grid[y][79] = true;
                }
            }
            
            // Mark cells that are reachable from outside
            while let Some((y, x)) = queue.pop() {
                // Mark current cell as visited in the fill grid
                fill_grid[y][x] = true;
                
                // Check all 8 directions
                for (dy, dx) in Self::DIRS {
                    let ny = (y as i32 + dy) as usize;
                    let nx = (x as i32 + dx) as usize;
                    
                    if ny < 25 && nx < 80 && !fill_grid[ny][nx] {
                        fill_grid[ny][nx] = true;
                        queue.push((ny, nx));
                    }
                }
            }
            
            // Check if we found any cells to fill
            let mut found_territory = false;
            'outer: for y in 0..25 {
                for x in 0..80 {
                    if !fill_grid[y][x] && matches!(self.board[y][x], Cell::Empty) {
                        found_territory = true;
                        break 'outer;
                    }
                }
            }
            
            // Only convert tails if we actually filled some territory
            if found_territory {
                // First convert all tails to solid for this player
                for y in 0..25 {
                    for x in 0..80 {
                        if let Cell::Tail(p, _) = self.board[y][x] {
                            if p == pid {
                                self.board[y][x] = Cell::Solid(pid, true);
                            }
                        }
                        if let Cell::Head(p, _) = self.board[y][x] {
                            if p == pid {
                                self.board[y][x] = Cell::Head(pid, true);
                            }
                        }
                    }
                }
            }
            
            // Convert all unreached cells to territory
            for y in 0..25 {
                for x in 0..80 {
                    if !fill_grid[y][x] {
                        self.board[y][x] = Cell::Solid(pid, true);
                    }
                }
            }
        }
    }

    fn clear_player_territory(&mut self, player_id: u8) {
        for y in 0..25 {
            for x in 0..80 {
                if let Cell::Head(pid, _) | Cell::Tail(pid, _) | Cell::Solid(pid, _) = self.board[y][x] {
                    if pid == player_id {
                        self.board[y][x] = Cell::Empty;
                    }
                }
            }
        }
    }
}