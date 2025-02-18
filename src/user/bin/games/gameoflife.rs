use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use alloc::boxed::Box;
use crate::std::application::{Application, Error};
use async_trait::async_trait;
use crate::std::render::{ColouredChar, Dimensions, Frame, Position, RenderError, Window};
use crate::std::io::{KeyStroke, Stdin, Color, ColorCode, Display};
use crate::std::time::wait;

pub struct GameOfLife {
    frame: Frame
}

const LOOP_SPEED: f64 = 0.1;

#[async_trait]
impl Application for GameOfLife {
    fn new() -> Self {
        Self {
            frame: Frame::new(Position::new(0, 0), Dimensions::new(80, 25)).unwrap()
        }
    }
    async fn run(&mut self, window: Option<Window>, _args: Vec<String>) -> Result<(), Error> {
        // setup:
        let _d = Display::borrow();

        let xoffset = 38;
        let yoffset = 5;

        // example pattern
        self.activate(0 + xoffset, 1 + yoffset);
        self.activate(1 + xoffset, 0 + yoffset);
        self.activate(1 + xoffset, 1 + yoffset);
        self.activate(2 + xoffset, 1 + yoffset);

        self.activate(0 + xoffset, 4 + yoffset);
        self.activate(1 + xoffset, 4 + yoffset);
        self.activate(2 + xoffset, 4 + yoffset);

        self.activate(0 + xoffset, 6 + yoffset);
        self.activate(2 + xoffset, 6 + yoffset);
        self.activate(0 + xoffset, 7 + yoffset);
        self.activate(2 + xoffset, 7 + yoffset);

        self.activate(0 + xoffset, 9 + yoffset);
        self.activate(1 + xoffset, 9 + yoffset);
        self.activate(2 + xoffset, 9 + yoffset);

        self.activate(0 + xoffset, 12 + yoffset);
        self.activate(1 + xoffset, 13 + yoffset);
        self.activate(1 + xoffset, 12 + yoffset);
        self.activate(2 + xoffset, 12 + yoffset);

        self.mainloop()?;

        Ok(())
    }
}

impl GameOfLife {
    fn activate(&mut self, x: u8, y: u8) {
        self.frame[24 - y as usize][x as usize] = ColouredChar::coloured('#', ColorCode::new(Color::Green, Color::Black));
    }
    fn mainloop(&mut self) -> Result<(), Error> {
        'mainloop: loop {
            // render element previous frame before resetting.

            wait(LOOP_SPEED);

            self.render().map_err(|_| Error::ApplicationError(String::from("failed to render game screen")))?;
            match Stdin::try_keystroke() {
                Some(KeyStroke::Char('x')) => break 'mainloop,
                _ => {},
            }

            // TODO: Logic goes here

            let mut frame = Frame::new(Position::new(0, 0), Dimensions::new(80, 25)).unwrap();

            self.frame.frame.iter().enumerate().for_each(|(y, row)| row.iter().enumerate().for_each(|(x, _chr)| {
                frame[y][x] = self.get_new_value(x as u8, y as u8);
            }));

            self.frame = frame;
        }
        Ok(())
    }

    fn get_new_value(&self, x: u8, y: u8) -> ColouredChar {
        let adjacent = vec![(0i32, 1i32), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)].into_iter().map(|(relx, rely)| {
            (x as i32 + relx, y as i32 + rely)
        }).filter(|(absx, absy)|  {
            0 <= *absx && *absx < 80 && 0 <= *absy && *absy < 25
        }).collect::<Vec<(i32, i32)>>();

        let alive = adjacent.iter().filter(|(x, y)| self.frame[*y as usize][*x as usize] == ColouredChar::coloured('#', ColorCode::new(Color::Green, Color::Black))).count();

        if alive == 2 {
            if self.frame[y as usize][x as usize] == ColouredChar::coloured('#', ColorCode::new(Color::Green, Color::Black)) {
                return ColouredChar::coloured('#', ColorCode::new(Color::Green, Color::Black));
            } else {
                return ColouredChar::null();
            }
        } else if alive == 3 {
            ColouredChar::coloured('#', ColorCode::new(Color::Green, Color::Black))
        } else {
            ColouredChar::null()
        }
    }

    fn render(&self) -> Result<(), RenderError> {
        self.frame.write_to_screen()?;
        Ok(())
    }
}














