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
use crate::{
    buffer::Buffer,
    units::Size
};

#[derive(Clone, Debug)]
pub enum TetroState {
    Descent,
    WaitToPlace
}

#[derive(Clone, Debug)]
pub struct Tetro {
    pub state: TetroState,
    pub buffer: Buffer
}

impl Tetro {
    pub fn rotate_right(&mut self) -> Tetro {
        Tetro {
            state: TetroState::Descent,
            buffer: self.buffer.rotate_right()
        }
    }

    pub fn size(&self) -> Size {
        self.buffer.size()
    }

    pub fn new() -> Tetro {
        let mut tetro = Tetro::tetros();

        let mut rng = thread_rng();
        let mut rotate = rng.gen_range(0..4);
        while rotate >= 0 {
            tetro.rotate_right();
            rotate -= 1;
        }

        tetro
    }

    pub fn from_vec(buffer: Vec<Vec<Option<Color>>>) -> Tetro {
        Tetro {
            state: TetroState::Descent,
            buffer: Buffer::from_vecs(buffer)
        }
    }

    fn tetros() -> Tetro {
        const TETRO_LEN: usize = 7;

        let fns: [fn() -> Tetro; TETRO_LEN] = [
            || -> Tetro { Tetro::from_vec(vec![
                vec![Some(Color::Cyan), Some(Color::Cyan), Some(Color::Cyan), Some(Color::Cyan)],
            ])},
            || -> Tetro { Tetro::from_vec(vec![
                vec![Some(Color::Blue), None, None],
                vec![Some(Color::Blue), Some(Color::Blue), Some(Color::Blue)]
            ])},
            || -> Tetro { Tetro::from_vec(vec![
                vec![None, None, Some(Color::LightMagenta)],
                vec![Some(Color::LightMagenta), Some(Color::LightMagenta), Some(Color::LightMagenta)]
            ])},
            || -> Tetro { Tetro::from_vec(vec![
                vec![Some(Color::Yellow), Some(Color::Yellow)],
                vec![Some(Color::Yellow), Some(Color::Yellow)]
            ])},
            || -> Tetro { Tetro::from_vec(vec![
                vec![None, Some(Color::Green), Some(Color::Green)],
                vec![Some(Color::Green), Some(Color::Green), None]
            ])},
            || -> Tetro { Tetro::from_vec(vec![
                vec![None, Some(Color::Magenta), None],
                vec![Some(Color::Magenta), Some(Color::Magenta), Some(Color::Magenta)]
            ])},
            || -> Tetro { Tetro::from_vec(vec![
                vec![Some(Color::Red), Some(Color::Red), None],
                vec![None, Some(Color::Red), Some(Color::Red)]
            ])}
        ];

        let mut rng = thread_rng();
        let i = rng.gen_range(0..TETRO_LEN);
        fns[i]()
    }
}
