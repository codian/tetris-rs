use tui::{
    backend::Backend,
    terminal::{Terminal, Frame},
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Modifier},
    text::{Span, Spans},
    widgets::{Borders, BorderType, Block, Paragraph},
};
use rand::{Rng, thread_rng};
use tui::widgets::Widget;

#[derive(Debug)]
pub struct Pos {
  pub x: u16,
  pub y: u16
}

impl Pos {
    pub fn new(x: u16, y: u16) -> Pos {
        Pos { x, y }
    }
}

#[derive(Debug)]
pub struct Size {
  pub width: u16,
  pub height: u16
}

impl Size {
    pub fn mid_x(&self) -> u16 { self.width / 2 }
    pub fn mid_y(&self) -> u16 { self.height / 2 }

    pub fn new(width: u16, height: u16) -> Size {
        Size { width, height }
    }
}
