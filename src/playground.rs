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
    tetro::Tetro,
    buffer::Buffer,
    units::{Pos, Size}
};
use crate::playground::IntentResult::OutBound;

const SIZE: Size = Size { width: 12, height: 20 };
const MAX_DROP_SPEED: u32 = 10;

enum IntentResult {
    Ok(Pos),
    OutBound(Pos),
    ReachBottom(Pos)
}

#[derive(Debug)]
pub struct Playground {
    pub score: u32,

    pub tetro_pos: Pos,
    pub tetro: Option<Tetro>,
    pub next: Option<Tetro>,
    pub debug_msg: String,

    pub buffer: Buffer,

    // speed
    pub drop_speed: u32,
    pub tick_count: u32
}

impl Playground {
    pub fn size(&self) -> Size { SIZE }

    pub fn widget(&self) -> Paragraph {
        let buffer =
            if let Some(tetro) = &self.tetro {
                &self.buffer // TODO + tetro.buffer
            } else {
                &self.buffer
            };

        let spans: Vec<Spans> = buffer.to_spans();

        Paragraph::new(spans)
    }

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

            self.debug_msg = format!("{:?}", self.next);
        }

        // drop tetro soft
        self.tick_count += 1;
        if self.tick_count > (MAX_DROP_SPEED - self.drop_speed) {
            self.drop_soft();
        }
        self.debug_msg = format!("{:?}", self.tick_count);
    }

    pub fn on_keydown(&mut self, key: &KeyEvent) {
        match key.code {
            KeyCode::Left => {
                self.debug_msg = String::from("left");
                self.move_left();
            },
            KeyCode::Right => {
                self.debug_msg = String::from("right");
                self.move_right();
            },
            KeyCode::Up => {
                self.debug_msg = String::from("up");
                self.rotate_right();
            },
            KeyCode::Down => {
                self.debug_msg = String::from("down");
                self.drop_soft();
            },
            KeyCode::Char(char) if char == ' ' => {
                self.debug_msg = String::from("space");
                self.drop_hard();
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

            drop_speed: 1,
            tick_count: 0
        }
    }

    pub fn move_left(&mut self) {
        if self.tetro_pos.x == 0 { return }

        if let Some(mut tetro) = self.tetro.take() {
            let new_pos =
                match self.intend_to_move(self.tetro_pos.x - 1, self.tetro_pos.y, &tetro) {
                    IntentResult::Ok(pos) => pos,
                    IntentResult::ReachBottom(pos) => pos,
                    IntentResult::OutBound(pos) => pos
                };

            self.tetro = Some(tetro);
            self.tetro_pos = new_pos;
        }
    }

    pub fn move_right(&mut self) {
        if let Some(mut tetro) = self.tetro.take() {
            let new_pos =
                match self.intend_to_move(self.tetro_pos.x + 1, self.tetro_pos.y, &tetro) {
                    IntentResult::Ok(pos) => pos,
                    IntentResult::ReachBottom(pos) => pos,
                    IntentResult::OutBound(pos) => pos
                };

            self.tetro = Some(tetro);
            self.tetro_pos = new_pos;
        }
    }

    pub fn rotate_right(&mut self) {
        if let Some(mut tetro) = self.tetro.take() {
            let size = tetro.size();
            let new_tetro = tetro.rotate_right();
            let new_pos =
                match self.intend_to_move(self.tetro_pos.x + size.mid_x() - 1, self.tetro_pos.y + size.mid_y() - 1, &tetro) {
                    IntentResult::Ok(pos) => pos,
                    IntentResult::ReachBottom(pos) => pos,
                    IntentResult::OutBound(pos) => pos
                };

            self.tetro = Some(new_tetro);
            self.tetro_pos = new_pos;
        }
    }
    pub fn drop_soft(&mut self) {
        if let Some(mut tetro) = self.tetro.take() {
            self.tetro_pos =
                match self.intend_to_move(self.tetro_pos.x, self.tetro_pos.y + 1, &tetro) {
                    IntentResult::Ok(pos) => pos,
                    IntentResult::ReachBottom(pos) => pos,
                    IntentResult::OutBound(pos) => pos
                };
            self.tetro = Some(tetro);
            self.tick_count = 0; // initialize tick_count
        }
    }

    pub fn drop_hard(&mut self) {
        if let Some(mut tetro) = self.tetro.take() {
            self.tetro = Some(tetro);
        }
    }

    fn intend_to_move(&self, x: u16, y: u16, tetro: &Tetro) -> IntentResult {
        let pl_size = self.size();
        let size = tetro.size();
        if (x + size.width) > pl_size.width {
            IntentResult::OutBound(Pos::new(x - 1, y))
        } else {
            IntentResult::Ok(Pos::new(x, y))
        }
    }
}