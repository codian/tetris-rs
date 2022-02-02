use tui::{
    backend::Backend,
    terminal::{Terminal, Frame},
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Modifier},
    text::{Span, Spans},
    widgets::{Borders, BorderType, Block, Paragraph},
};
use crossterm::{
    execute,
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen,
               LeaveAlternateScreen}
};
use crate::{
    tetro::{Tetro, TetroState},
    buffer::Buffer,
    units::{Pos, Size}
};

const SIZE: Size = Size { width: 12, height: 20 };
const MAX_DESCENT_SPEED: u32 = 10;

#[derive(Debug)]
pub struct Playground {
    pub score: u32,

    pub tetro_pos: Pos,
    pub tetro: Option<Tetro>,
    pub next: Option<Tetro>,
    pub debug_msg: String,

    pub buffer: Buffer,

    // speed
    pub descent_speed: u32,
    pub tick_count: u32
}

impl Playground {
    pub fn size(&self) -> Size { SIZE }

    pub fn on_tick(&mut self) {
        // new next and tetro
        if self.next.is_none() {
            self.next = Some(Tetro::new());
        } else if self.tetro.is_none() {
            self.tetro = if let Some(new) = self.next.take() {
                let pl_size = self.size();
                let new_size = new.size();
                self.tetro_pos = Pos::new(pl_size.mid_x() - new_size.mid_x(), 0);
                Some(new)
            } else {
                None
            };

            // self.debug_msg = format!("{:?}", self.next);
        }

        // descent tetro soft
        self.tick_count += 1;
        if self.tick_count > (MAX_DESCENT_SPEED - self.descent_speed) {
            self.descend_soft();
        }
        // self.debug_msg = format!("{:?}", self.tick_count);
    }

    pub fn on_keydown(&mut self, key: &KeyEvent) {
        match key.code {
            KeyCode::Left => {
                // self.debug_msg = String::from("left");
                self.move_left();
            },
            KeyCode::Right => {
                // self.debug_msg = String::from("right");
                self.move_right();
            },
            KeyCode::Up => {
                // self.debug_msg = String::from("up");
                self.rotate_right();
            },
            KeyCode::Down => {
                // self.debug_msg = String::from("down");
                self.descend_soft();
            },
            KeyCode::Char(char) if char == ' ' => {
                // self.debug_msg = String::from("space");
                self.descend_hard();
            },
            _ => {}
        }
    }

    pub fn new() -> Playground {
        Playground {
            buffer: Buffer::new(SIZE),
            score: 0,
            tetro_pos: Pos::new(0, 0),
            tetro: None,
            next: None,
            debug_msg: String::from(""),

            descent_speed: 1,
            tick_count: 0
        }
    }

    pub fn move_left(&mut self) {
        if self.tetro_pos.x == 0 { return }

        if let Some(mut tetro) = self.tetro.take() {
            self.move_to(self.tetro_pos.x - 1, self.tetro_pos.y, &mut tetro);
            self.tetro = Some(tetro);
        }
    }

    pub fn move_right(&mut self) {
        if let Some(mut tetro) = self.tetro.take() {
            self.move_to(self.tetro_pos.x + 1, self.tetro_pos.y, &mut tetro);
            self.tetro = Some(tetro);
        }
    }

    pub fn rotate_right(&mut self) {
        if let Some(mut tetro) = self.tetro.take() {
            let new_tetro = tetro.rotate_right();
            let size = tetro.size();
            self.move_to(self.tetro_pos.x + size.mid_x() - 1, self.tetro_pos.y + size.mid_y() - 1, &mut tetro);
            self.tetro = Some(new_tetro);
        }
    }
    pub fn descend_soft(&mut self) {
        if let Some(mut tetro) = self.tetro.take() {
            match tetro.state {
                TetroState::Descent => {
                    self.move_to(self.tetro_pos.x, self.tetro_pos.y + 1, &mut tetro);
                    self.tetro = Some(tetro);
                },
                TetroState::WaitToPlace => {
                    self.place(self.tetro_pos.x, self.tetro_pos.y, &tetro);
                }
            }
            self.tick_count = 0; // initialize tick_count
        }
    }

    pub fn descend_hard(&mut self) {
        if let Some(mut tetro) = self.tetro.take() {
            self.tetro = Some(tetro);
        }
    }

    fn place(&mut self, x: u16, y: u16, tetro: &Tetro) {
        let Size { width, height } = tetro.buffer.size();
        let Pos { x: tetro_x, y: tetro_y } = self.tetro_pos;
        for y in 0..height {
            for x in 0..width {
                if let Some(color) = tetro.buffer.get(x, y) {
                    self.buffer.set(x + tetro_x, y + tetro_y, Some(color));
                }
            }
        }
    }

    fn move_to(&mut self, x: u16, y: u16, tetro: &mut Tetro) {
        let pl_size = self.size();
        let size = tetro.size();
        if (x + size.width) > pl_size.width { // over right bound
            self.tetro_pos = Pos::new(x - 1, y);
            // self.debug_msg = format!("{:?}", self.tetro_pos);
            tetro.state = TetroState::Descent;
        } else if (y + size.height) > pl_size.height { // over bottom bound
            self.tetro_pos = Pos::new(x, y - 1);
            // self.debug_msg = format!("{:?}", self.tetro_pos);
            tetro.state = TetroState::WaitToPlace;
        } else if self.is_reach_bottom(x, y, &tetro) { // reach bottom
            self.tetro_pos = Pos::new(x, y - 1);
            // self.debug_msg = format!("{:?}", self.tetro_pos);
            tetro.state = TetroState::WaitToPlace;
        } else { // in bound
            self.tetro_pos = Pos::new(x, y);
            // self.debug_msg = format!("{:?}", self.tetro_pos);
            tetro.state = TetroState::Descent;
        }
    }

    fn is_reach_bottom(&self, pos_x: u16, pos_y: u16, tetro: &Tetro) -> bool {
        let Size { width, height } = tetro.buffer.size();
        for ty in 0..height {
            for tx in 0..width {
                if let Some(_) = tetro.buffer.get(tx, ty) {
                    if self.buffer.get(pos_x + tx, pos_y + ty).is_some() {
                        return true
                    }
                }
            }
        }
        false
    }
}