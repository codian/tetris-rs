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
    playground::Playground,
    tetro::Tetro,
    units::{Pos, Size}
};

#[derive(Debug)]
pub enum State {
    Ready,
    Playing,
    Finished
}

#[derive(Debug)]
pub struct Screen {
    pub state: State,
    pub pos: Pos,
    pub playground: Playground,
    pub debug_msg: String
}

pub const BG_COLOR: Color = Color::Black;
pub const SIZE: Size = Size { width: 27, height: 24 };

impl Screen {
    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        // setup pos
        let f_size = f.size();
        let f_center = Size::new(f_size.width / 2, f_size.height / 2);
        self.pos = Pos::new(f_center.width - SIZE.mid_x(), f_center.height - SIZE.mid_y());

        // entire area
        let block = Block::default()
            .style(Style::default().bg(Color::Gray));
        f.render_widget(block, self.rect(0, 0, SIZE.width, SIZE.height));

        // score
        let score = Paragraph::new(Spans::from(vec![
            Span::styled("SCORE: ",Style::default().add_modifier(Modifier::ITALIC)),
            Span::styled(format!("{}", self.playground.score),Style::default().add_modifier(Modifier::BOLD)),
        ])).alignment(Alignment::Center);
        f.render_widget(score, self.rect(
            0, 0, SIZE.width, 1));

        // playground frame
        let pl_size = self.playground.size();
        let block = Block::default()
            .borders(Borders::ALL)
            .title("< TETRIS >")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Thick);
        let Size {width, height} = self.playground.size();
        f.render_widget(block, self.rect(
            0, 2, pl_size.width + 2, pl_size.height + 2));

        // playground
        let widget = Paragraph::new(self.playground.buffer.to_spans());
        f.render_widget(widget, self.rect(
            1, 3, pl_size.width, pl_size.height));

        // next frame
        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Thick)
            .title(Span::styled("NEXT",Style::default()))
            .title_alignment(Alignment::Center);
        f.render_widget(block, self.rect(
            1 + pl_size.width + 2 + 1, 2, 6, 6));

        // next
        if let Some(next) = &self.playground.next {
            let next_size = next.size();
            let widget = Paragraph::new(next.buffer.to_spans());
            f.render_widget(widget, self.rect(
                1 + pl_size.width + 2 + 1 + 1, 3, next_size.width, next_size.height));
        }

        // tetro
        if let Some(tetro) = &self.playground.tetro {
            let pos = &self.playground.tetro_pos;
            let size = tetro.size();
            let widget = Paragraph::new(tetro.buffer.to_spans());
            f.render_widget(widget, self.rect(
                1 + pos.x, 3 + pos.y, size.width, size.height));
        }

        // press space key
        match self.state {
            State::Ready => {
                let press_space_key = Paragraph::new(
                    Spans::from(vec![
                        Span::styled(
                            " PRESS SPACE KEY! ",
                            Style::default().bg(Color::Blue)
                        )
                    ])
                ).alignment(Alignment::Center);
                f.render_widget(press_space_key, self.rect(
                    0, SIZE.mid_y(), SIZE.width, 1
                ));
            },
            _ => {}
        }

        // debug_msg
        let debug_msg = Paragraph::new(
            Spans::from(vec![
                Span::raw(self.debug_msg.clone()),
                Span::raw(self.playground.debug_msg.clone()),
            ])
        ).alignment(Alignment::Left);
        f.render_widget(debug_msg, self.rect(
            0, SIZE.height, SIZE.width, 1
        ));
    }

    pub fn on_tick(&mut self) {
        match self.state {
            State::Ready => {
            },
            State::Finished => {
            },
            State::Playing => {
                self.playground.on_tick();
            }
        }
    }

    pub fn on_keydown(&mut self, key: &KeyEvent) {
        match self.state {
            State::Ready => match key.code {
                KeyCode::Char(char) if char == ' ' => {
                    self.state = State::Playing;
                },
                _ => {}
            },
            State::Finished => match key.code {
                KeyCode::Char(char) if char == ' ' => {
                    self.state = State::Ready;
                },
                _ => {}
            },
            State::Playing => {
                self.playground.on_keydown(key)
            }
        }
    }

    pub fn new() -> Screen {
        Screen {
            state: State::Ready,
            pos: Pos::new(0, 0),
            playground: Playground::new(),
            debug_msg: String::from("")
        }
    }

    pub fn rect(&self, x: u16, y: u16, width: u16, height: u16) -> Rect {
        Rect {
            x: x + self.pos.x,
            y: y + self.pos.y,
            width,
            height
        }
    }
}
