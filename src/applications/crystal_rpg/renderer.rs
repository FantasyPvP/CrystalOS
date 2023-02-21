use lazy_static::lazy_static;
use spin::Mutex;
use alloc::{vec::Vec, string::String};
use crate::std::{println, serial_println};

use crate::render;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

lazy_static! {
    pub static ref RENDERER: Mutex<Renderer> = Mutex::new(Renderer::new() );
}

#[derive(Clone)]
pub struct Element {
    frame: Vec<Vec<char>>,
    dimensions: (u8, u8)
}
impl Element {
    pub fn from_str(elemstr: String) -> Self {
        let mut element = Element { frame: Vec::<Vec<char>>::new(), dimensions: (0, 0) }; 

        for line in elemstr.split("\n") {
            let mut ln = Vec::<char>::new();
            for col in line.chars() {
                ln.push(col)
            };
            element.frame.push(ln);
        }

        for row in element.clone().frame {
            let n = row.len();
            if n > element.dimensions.0 as usize {
                element.dimensions.0 = n as u8;
            }
        }
        element
    }

    pub fn render(&mut self,  pos: (u8, u8)) { // x,y
        for (i, row) in self.frame.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                println!("{} {} {}", i, j, col);
                RENDERER.lock().frame[i + pos.1 as usize][j + pos.0 as usize] = *col;
            };
        }
    }
}

#[derive(Clone, Copy)]
pub struct Renderer {
    frame: [ [ char; BUFFER_WIDTH ]; BUFFER_HEIGHT],
}


impl Renderer {
    pub fn render_frame(&self) {
        render::RENDERER.lock().render_frame(self.frame)
    }

    fn new() -> Self {
        let mut frame: [[char; BUFFER_WIDTH]; BUFFER_HEIGHT] = [[' '; BUFFER_WIDTH]; BUFFER_HEIGHT];
        for i in 0..BUFFER_WIDTH {
            frame[0][i] = "┌──────────────────────────────────────────────────────────────────────────────┐".chars().collect::<Vec<char>>()[i];
            frame[BUFFER_HEIGHT -1][i] = "└──────────────────────────────────────────────────────────────────────────────┘".chars().collect::<Vec<char>>()[i];
        }
        
        for j in 1..BUFFER_HEIGHT -1 {
            for i in 0..BUFFER_WIDTH {
                frame[j][i] = "│                                                                              │".chars().collect::<Vec<char>>()[i];               
            }
        }

        Renderer { frame: frame }
    }

    pub fn get_frame(&self) -> &[ [ char; BUFFER_WIDTH ]; BUFFER_HEIGHT] {
        &self.frame
    }

}


impl core::fmt::Display for Renderer {
    fn fmt(&self, _: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        println!(" ");
        for row in &self.frame {
            println!("{}", row.iter().collect::<String>());
        };
        Ok(())
    }
}
