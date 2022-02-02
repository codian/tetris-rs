use std::{io, thread, time::{Duration, Instant}};
use tui::{
    terminal::Terminal,
    backend::CrosstermBackend,
    layout::Alignment,
    widgets::{Borders, Block, Paragraph},
    style::{Color, Style, Modifier},
    text::{Span, Spans},
};
use crossterm::{
    execute,
    event,
    event::Event,
    event::KeyCode,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, SetSize, ScrollUp}
};

use crate::screen;

pub struct App {
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    screen: screen::Screen
}

impl App {
    pub fn new() -> Result<App, io::Error> {
        enable_raw_mode().unwrap();
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen).unwrap();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.hide_cursor().unwrap();

        Ok(App {
            terminal,
            screen: screen::Screen::new()
        })
    }

    pub fn run(&mut self) -> io::Result<()> {
        let tick_rate = Duration::from_millis(100);
        let mut last_tick = Instant::now();
        let mut loop_count = 0u32;
        loop {
            // tick
            if last_tick.elapsed() >= tick_rate {
                self.screen.on_tick();
                last_tick = Instant::now();
            }

            // keydown
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if let KeyCode::Char('q') = key.code {
                        return Ok(());
                    } else {
                        self.screen.on_keydown(&key);
                    }

                    // consume remaining key events
                    while crossterm::event::poll(Duration::from_secs(0))? {
                        event::read()?;
                    }
                }
            }

            // draw
            self.terminal.draw(
                |frame| { self.screen.draw(frame) }
            ).unwrap();

            // consume time
            if let Some(dur) = tick_rate.checked_sub(last_tick.elapsed()) {
                thread::sleep(dur);
            }

            loop_count += 1;
            // self.screen.debug_msg = format!("{}", count);
        }
    }
}

impl Drop for App {
    fn drop(&mut self) {
        // restore terminal
        disable_raw_mode().unwrap();
        // execute!(self.terminal.backend_mut(), LeaveAlternateScreen).unwrap();
        self.terminal.show_cursor().unwrap();
    }
}
