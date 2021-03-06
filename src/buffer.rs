use std::fmt::{Display, Formatter};
use tui::{
    style::{Color, Style},
    text::{Span, Spans},
};
use crate::{
    units::{Pos, Size}
};

#[derive(Clone, Debug)]
pub struct Buffer {
    cells: Vec<Vec<Option<Color>>>
}

impl Buffer {
    pub fn size(&self) -> Size { Size::new(self.width(), self.height()) }
    pub fn height(&self) -> u16 { u16::try_from(self.cells.len()).unwrap() }
    pub fn width(&self) -> u16 {
        if let Some(first) = self.cells.first() {
            u16::try_from(first.len()).unwrap()
        } else {
            0
        }
    }

    pub fn set(&mut self, x: u16, y: u16, val: Option<Color>) {
        assert!((y as usize) < self.cells.len());
        assert!((x as usize) < self.cells[0].len());
        self.cells[y as usize][x as usize] = val;
    }

    pub fn get(&self, x: u16, y: u16) -> Option<Color> {
        assert!((y as usize) < self.cells.len());
        assert!((x as usize) < self.cells[0].len());
        self.cells[y as usize][x as usize]
    }

    pub fn line_completed(&self, y: u16) -> bool {
        for val in &self.cells[y as usize] {
            if val.is_none() {
                return false
            }
        }
        true
    }

    pub fn remove_and_prepend_line(&mut self, y: u16) {
        self.cells.remove(y as usize);
        self.cells.insert(0, vec![None; self.width() as usize]);
    }

    pub fn rotate_right(&mut self) -> Buffer {
        let size = self.size();
        let new_size = Size::new(size.height, size.width);
        let mut new_buf = Buffer::new(new_size);
        for y in 0..size.height {
            for x in 0..size.width {
                let val = self.get(x, y);
                if val.is_some() {
                    new_buf.set(size.height - 1 - y, x, val);
                }
            }
        }
        new_buf
    }

    pub fn to_spans(&self) -> Vec<Spans> {
        self.cells.iter().map(|row| {
            Spans::from(
                row.iter().map(|cell| {
                    if let Some(color) = cell {
                        Span::styled("???", Style::default().fg(color.clone()))
                    } else {
                        Span::styled(" ", Style::default())
                    }
                }).collect::<Vec<Span>>()
            )
        }).collect::<Vec<Spans>>()
    }

    pub fn new(size: Size) -> Buffer {
        Buffer {
            cells: vec![vec![None; size.width as usize]; size.height as usize]
        }
    }

    pub fn from_vecs(cells: Vec<Vec<Option<Color>>>) -> Buffer {
        Buffer {
            cells
        }
    }
}

impl Display for Buffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut count = 0;
        let width = self.width();
        let height = self.height();
        for y in 0..height {
            for x in 0..width {
                if let Some(_) = self.get(x, y) {
                    count += 1;
                }
            }
        }
        write!(f, "Cells count: {}", count)
    }
}
