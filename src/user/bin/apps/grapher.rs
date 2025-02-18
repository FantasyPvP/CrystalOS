use alloc::string::{String, ToString};
use alloc::{format, vec};
use alloc::vec::Vec;
use alloc::boxed::Box;
use core::any::Any;
use async_trait::async_trait;
use crate::std::application::{Application, Error};
use crate::std::render::{ColouredChar, Dimensions, Frame, Position, RenderError, Window};
use crate::std::io::{Display, KeyStroke, Stdin};

use crate::user::lib::libgui::{
    cg_core::{CgComponent},
    cg_widgets::CgContainer,
    cg_inputs::CgLineEdit,
};
use crate::user::lib::libgui::cg_core::{CgTextEdit, Widget};

use super::calc;

const OFFSET_X: i64 = 39;
const OFFSET_Y: i64 = 10;

use core::f64::consts::E;
use core::f64::consts::PI;


#[derive(Clone)]
pub struct Grapher {
    points: Vec<PointF64>,
    frame: Frame,
}

#[derive(Clone, Debug)]
struct PointF64 {
    x: f64,
    y: f64,
}
struct PointI64 {
    x: i64,
    y: i64,
}


#[async_trait]
impl Application for Grapher {
    fn new() -> Self {
        Self {
            points: Vec::new(),
            frame: Frame::new(Position::new(1, 1), Dimensions::new(78, 22)).unwrap()
        }
    }
    async fn run(&mut self, window: Option<Window>, args: Vec<String>) -> Result<(), Error> {
        let _d = Display::borrow();

        self.frame.frame = vec![vec![ColouredChar::new(' '); self.frame.dimensions.x]; self.frame.dimensions.y];

        if args.len() > 0 {
            let equation: String = args.into_iter().collect();
            self.graph_equation(equation, (0, 0));

            if let Ok(frame) = self.render() {
                frame.write_to_screen().map_err(|_| Error::ApplicationError(String::from("failed to write to screen")))?;
            }

            loop {
                match Stdin::keystroke().await {
                    KeyStroke::Char('x') => break,
                    _ => continue,
                }
            }

            return Ok(());
        }
        else {
            let mut container = CgContainer::new(
                Position::new(0, 0),
                Dimensions::new(80, 25),
                true,
            );

            container.insert("entry_box", Widget::insert(CgLineEdit::new(
                Position::new(1, 23),
                78,
                String::from("function >")
            )));
            container.insert("grapher", Widget::insert(self.clone()));

            let mut commandresult = String::new();

            let mut offset_x: i64 = 0;
            let mut offset_y: i64 = 0;

            let mut redraw_graph = true;

            while let c = Stdin::keystroke().await {

                let entry_widget = container.elements.get("entry_box").unwrap();
                let mut entry = entry_widget.fetch::<CgLineEdit>().unwrap();

                redraw_graph = true;
                match c {
                    KeyStroke::Char('\n') => {
                        commandresult = entry.text.iter().collect();
                        entry.clear();
                        offset_x = 0;
                        offset_y = 0;
                    },
                    KeyStroke::Char(Stdin::BACKSPACE) => {
                        redraw_graph = false;
                        entry.backspace()
                    },
                    KeyStroke::Char('`') => {
                        break;
                    }
                    KeyStroke::Char(c) => {
                        redraw_graph = false;
                        entry.write_char(c)
                    },
                    KeyStroke::Left => offset_x -= 1,
                    KeyStroke::Right => offset_x += 1,
                    KeyStroke::Up => offset_y -= 1,
                    KeyStroke::Down => offset_y += 1,
                    KeyStroke::Alt => break,
                    _ => {
                        redraw_graph = false;
                    }
                }

                if commandresult.len() > 0 && redraw_graph {
                    self.reset_frame();
                    self.graph_equation(commandresult.clone(), (offset_x, offset_y));
                    let self_widget = container.elements.get("grapher").unwrap();
                    self_widget.update(self.clone());
                }

                entry_widget.update(entry);

                if let Ok(frame) = container.render() {

                    let self_widget = container.elements.get("grapher").unwrap();
                    let _self_clone = self_widget.fetch::<Grapher>().unwrap();

                    let entry = container.elements.get("entry_box").unwrap();
                    let _entry_clone = entry.fetch::<CgLineEdit>().unwrap();

                    frame.write_to_screen().map_err(|_| Error::ApplicationError(String::from("failed to write to screen")))?;
                }
            }

        }
        Ok(())
    }
}

impl Grapher {

    fn graph_equation(&mut self, equation: String, offsets: (i64, i64)) {

        let cal = calc::Calculator::new();
        let ast = cal.get_expr(equation.chars().map(|c| {
            match c {
                'e' => format!("({})", E),
                'π' => format!("({})", PI),
                _ => c.to_string(),
            }
        }).collect::<String>());
        
        if let Ok(ast) = ast {
            for x in -4000..4000 {
                let x = x as f64 / 100.0;

                let mut cmd = ast.clone();
                cal.substitute(&mut cmd, x + offsets.0 as f64);

                //
                // let new_eq = equation.chars().map(|c| {
                //     match c {
                //         'x' => format!("({})", x + offsets.0 as f64),
                //         'e' => format!("({})", E),
                //         'π' => format!("({})", PI),
                //         _ => c.to_string(),
                //     }
                // }).collect::<String>();

                let fx = cal.evaluate(&cmd);
                if let Ok(y) = fx {
                    self.render_point(PointF64 {
                        x,
                        y: y + offsets.1 as f64,
                    })
                }
            };
        }
    }


    fn render_point(&mut self, point: PointF64) {
        let point = PointI64 {
            x: point.x as i64,
            y: point.y as i64,
        };

        if point.x < -39 || point.x >= 39 || point.y < -10 || point.y >= 12 {
            return;
        }

        let offset_x = point.x + OFFSET_X;
        let offset_y = point.y + OFFSET_Y;
        self.frame.write(Position::new(offset_x as usize, 21-offset_y as usize), ColouredChar::new('*')).expect("Failed to write to frame - this function is broken.");
    }

    fn reset_frame(&mut self) {
        self.frame.frame = vec![vec![ColouredChar::new(' '); self.frame.dimensions.x]; self.frame.dimensions.y];
    }
}

impl CgComponent for Grapher {
    fn render(&self) -> Result<Frame, RenderError> {
        Ok(self.frame.clone())
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}












