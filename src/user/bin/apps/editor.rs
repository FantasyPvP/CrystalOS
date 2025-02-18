use crate::{serial_println, std};
use crate::std::application::{self, Application};
use crate::std::io::{Color, ColorCode, Display, KeyStroke};
use crate::std::render::{ColouredChar, Frame, Position, RenderError, Window};
use crate::user::lib::libgui::cg_core::CgComponent;




use alloc::format;
use alloc::{string::{String, ToString}, vec::Vec, boxed::Box};

use async_trait::async_trait;


pub struct Editor {
    buffer: Vec<Vec<char>>, // outer vec is the line, inner is col;
    cursor_pos: Position<i32>,
    offset_pos: Position<i32>,
    command: String,
    mode: Mode,
    unsaved: bool,
    display: Display,
    lineno_width: i32,
}

enum Mode {
    Normal,
    Insert,
    Command,
    Diff // TODO
}

impl core::fmt::Display for Mode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Mode::Normal => write!(f, "Normal"),
            Mode::Insert => write!(f, "Insert"),
            Mode::Command => write!(f, "Commnd"),
            Mode::Diff => write!(f, "Diff  "),
        }
    }
}

impl Editor {
    fn move_cursor(&mut self, dx: i32, dy: i32) {
        if dy != 0 
            && self.cursor_pos.y + dy >= 0 
            && self.cursor_pos.y + dy <= self.buffer.len() as i32 
        {
            self.cursor_pos.y += dy;
            let line_width = self.buffer.get(self.cursor_pos.y as usize).unwrap_or(&Vec::<char>::new()).len() as i32;
            if self.cursor_pos.x > line_width {
                self.cursor_pos.x = line_width;
            }
        } else if self.cursor_pos.x + dx < 0 {
            if self.cursor_pos.y - 1 >= 0 {
                self.cursor_pos.y -= 1;
                self.cursor_pos.x = self.buffer.get(self.cursor_pos.y as usize).unwrap_or(&Vec::<char>::new()).len() as i32;
            }
        } else if self.cursor_pos.x + dx > self.buffer.get(self.cursor_pos.y as usize).unwrap_or(&Vec::<char>::new()).len() as i32 {
            if self.cursor_pos.y + 1 <= self.buffer.len() as i32 {
                self.cursor_pos.x = 0;
                self.cursor_pos.y += 1;
            }
        } else if dx != 0 {
            self.cursor_pos.x += dx;
        }

        serial_println!("cursor: {} {} offset: {} {} ", self.cursor_pos.x, self.cursor_pos.y, self.offset_pos.x, self.offset_pos.y);

        while self.cursor_pos.x + 3 + (self.lineno_width + 2) > 80 + self.offset_pos.x {
            self.offset_pos.x += 1;
        }

        while self.cursor_pos.x - 3 < self.offset_pos.x && self.offset_pos.x - 3 >= 0 {
            self.offset_pos.x -= 1;
        }

        while self.cursor_pos.y + 3 > self.offset_pos.y + 25 {
            self.offset_pos.y += 1;
        }

        while self.cursor_pos.y - 3 < self.offset_pos.y && self.offset_pos.y - 3 >= 0 {
            self.offset_pos.y -= 1;
        }

        serial_println!(
            "moving cursor to {}, {}",
            (self.cursor_pos.x - self.offset_pos.x + self.lineno_width + 2) as u8,
            (self.cursor_pos.y - self.offset_pos.y) as u8
        );

        // print all the values below
        serial_println!("offset: {}, {}", self.offset_pos.x, self.offset_pos.y);
        serial_println!("cursor: {}, {}", self.cursor_pos.x, self.cursor_pos.y);
        serial_println!("line width: {}", self.lineno_width + 2);

        self.display.mv_cursor(
            (self.cursor_pos.x - self.offset_pos.x + self.lineno_width + 2) as u8, 
            (self.cursor_pos.y - self.offset_pos.y) as u8
        ).unwrap();
    }

    fn delete_char(&mut self) {
        self.unsaved = true;
        // if the cursor is at the end of the line
        if self.cursor_pos.x == self.buffer.get(self.cursor_pos.y as usize).unwrap_or(&Vec::<char>::new()).len() as i32 {
            
            if self.cursor_pos.y + 1 == self.buffer.len() as i32 {
                return;
            }

            let old_line = self.buffer[self.cursor_pos.y as usize + 1].clone();
            self.buffer[self.cursor_pos.y as usize].extend(&old_line);
            self.buffer.remove(self.cursor_pos.y as usize + 1);
        } else {
            self.buffer[self.cursor_pos.y as usize].remove(self.cursor_pos.x as usize);
        }
    }


    fn splitline(&mut self) {
        self.unsaved = true;

        if let Some(_) = self.buffer.get(self.cursor_pos.y as usize) {
            let first_half = self.buffer[self.cursor_pos.y as usize][..self.cursor_pos.x as usize].to_vec();
            let second_half = self.buffer[self.cursor_pos.y as usize][self.cursor_pos.x as usize..].to_vec();

            self.buffer[self.cursor_pos.y as usize] = first_half;
            self.buffer.insert(self.cursor_pos.y as usize + 1, second_half);
        } else {
            self.buffer.push(Vec::new());
        }
        
        self.move_cursor(1, 0);
    }

    fn insert_char(&mut self, c: char) {
        self.unsaved = true;

        if let Some(line) = self.buffer.get_mut(self.cursor_pos.y as usize) {
            line.insert(self.cursor_pos.x as usize, c);
        } else {
            self.buffer.push(Vec::new());
            self.buffer.get_mut(self.cursor_pos.y as usize).unwrap().push(c);
        }
    }
}

impl ToString for Editor {
    fn to_string(&self) -> String {
        self.buffer.iter().map(|line| line.iter().collect::<String>()).collect::<Vec<String>>().join("\n")
    }
}

#[async_trait]
impl Application for Editor {
    fn new() -> Editor {
        Editor {
            buffer: Vec::new(),
            cursor_pos: Position::zero(),
            offset_pos: Position::zero(),
            command: String::new(),
            mode: Mode::Normal,
            unsaved: false,
            display: Display::borrow(),
            lineno_width: 0
        }
    }

    async fn run(&mut self, window: Option<Window>, _args: Vec<String>) -> Result<(), application::Error> {

        // if let Some(s) = args.get(0) {
        //     self.buffer = s.lines().map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>()
        // }

        self.buffer = String::from("
    /$$ /$$$$$$$$ /$$   /$$  /$$$$$$  /$$$$$$$       /$$ /$$  /$$
   /$$/|_____ $$ | $$  / $$ /$$__  $$| $$____/      /$$/|  $$|  $$
  /$$/      /$$/ |  $$/ $$/| $$  \\ $$| $$          /$$/  \\  $$\\  $$
 /$$/      /$$/   \\  $$$$/ | $$  | $$| $$$$$$$    /$$/    \\  $$\\  $$
|  $$     /$$/     >$$  $$ | $$  | $$|_____  $$  /$$/      /$$/ /$$/
 \\  $$   /$$/     /$$/\\  $$| $$/$$ $$ /$$  \\ $$ /$$/      /$$/ /$$/
  \\  $$ /$$$$$$$$| $$  \\ $$|  $$$$$$/|  $$$$$$//$$/      /$$/ /$$/
   \\__/|________/|__/  |__/ \\____ $$$ \\______/|__/      |__/ |__/
                                 \\__/
    ").lines().map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>();

        

        loop {
            // start by rendering the screen
            self.lineno_width = self.buffer.len().to_string().len() as i32;
            self.render().unwrap().write_to_screen().unwrap();

            // wait for a keyboard input
            let keystroke = std::io::Stdin::keystroke().await;

            match self.mode {
                Mode::Normal => {
                    match keystroke {
                        KeyStroke::Char('i') => self.mode = Mode::Insert,
                        KeyStroke::Char(':') => self.mode = Mode::Command,
                        KeyStroke::Char('d') => self.mode = Mode::Diff,
                        KeyStroke::Char('`') => {
                            // TODO: End terminal session
                            // ncurses::endwin();
                            return Ok(());
                        }
                        _ => {}
                    }
                },
                Mode::Insert => {
                    match keystroke {
                        KeyStroke::Enter => {
                            // TODO: newline function
                        },
                        KeyStroke::Char(c) => {
                            match c {
                                // escape
                                '\x1B' => self.mode = Mode::Normal,
                                // delete
                                '\x7F' => self.delete_char(),
                                // backspace
                                '\x08' => {
                                    self.move_cursor(-1, 0);
                                    self.delete_char();
                                },
                                // enter
                                '\n' => self.splitline(),
                                _ => {
                                    self.insert_char(c);
                                    self.move_cursor(1, 0);
                                }
                            }
                        },
                        KeyStroke::Left => {
                            self.move_cursor(-1, 0);
                        },
                        KeyStroke::Right => {
                            self.move_cursor(1, 0);
                        },
                        KeyStroke::Up => {
                            self.move_cursor(0, -1);
                        },
                        KeyStroke::Down => {
                            self.move_cursor(0, 1);
                        },
                        KeyStroke::None => {
                            serial_println!("none");
                        },
                        _ => {
                            serial_println!("other");
                        }
                    }
                }
                Mode::Command => {
                    match keystroke {
                        KeyStroke::Enter => {
                            // TODO: execute command
                        },
                        KeyStroke::Char(c) => {
                            if c == '\x1B' { 
                                self.mode = Mode::Normal; 
                                self.command.clear();
                                continue;
                            }

                            self.command.push(c);
                        },
                        _ => {}
                    }
                }
                Mode::Diff => {}
            }
        }
    }
}

impl CgComponent for Editor {
    fn render(&self) -> Result<Frame, RenderError> {
        let mut frame = Frame::new(Position::zero(), Position::new(80, 25))?;
        let width = self.lineno_width as usize;
        let linecolour  = ColorCode::new(Color::Cyan, Color::Black);

        for (i, line) in (self.offset_pos.y..self.offset_pos.y + 24).enumerate() {
            if line >= self.buffer.len() as i32 {
                break;
            }

            // render the line numbers on the left hand side of the screen
            let line_num = format!("{:width$} │", line + 1);
            for (j, c) in line_num.chars().enumerate() {
                frame.write(Position::new(j, i), ColouredChar::coloured(c, linecolour))?;
            }   

            let line = self.buffer[line as usize].iter().collect::<String>();

            for (j, c) in line.chars().skip(self.offset_pos.x as usize).take(80 - (width + 2)).enumerate() {
                frame.write(Position::new(j + width + 2, i), ColouredChar::new(c))?;
            }
        }

        // render the toolbar

        // render mode (8 chars)
        let mode = format!("[{}]", self.mode);

        // render unsaved (10 chars)
        let unsaved = String::from(if self.unsaved { "[Unsaved!]" } else { "" });

        // line and col (variable width)
        let line_and_col = format!("[{}:{}] ", self.cursor_pos.y + 1, self.cursor_pos.x + 1);

        // write to screen
        let toolbar = line_and_col + " " + &mode + " " + &unsaved;

        for (i, c) in toolbar.chars().enumerate() {
            frame.write(Position::new(i, 24), ColouredChar::new(c))?;
        }
    
        Ok(frame)
    }
    
    fn as_any(&self) -> &dyn core::any::Any {
        self
    }
}

