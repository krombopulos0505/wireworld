use std::io::{Stdout, stdout, Write};
use crossterm::{
    terminal, queue,
    cursor,
    style::{Color, Print, ResetColor, SetBackgroundColor},
};
use crate::cell::Cell;
use crate::grid::Grid;

pub struct Renderer {
    pub out: Stdout,
}

impl Renderer {
    pub fn new() -> Self {
        Self { out: stdout() }
    }

    pub fn draw(&mut self, grid: &Grid<Cell>) -> std::io::Result<()> {
        queue!(self.out, terminal::Clear(terminal::ClearType::All))?;
        for y in 0..grid.h as i32 {
            for x in 0..grid.w as i32 {
                if let Some(cell) = grid.get(x, y) {
                    queue!(self.out, cursor::MoveTo(2*x as u16, y as u16))?;
                    match cell {
                        Cell::Empty => {}
                        Cell::Head => {
                            queue!(self.out, SetBackgroundColor(Color::Blue))?;
                            queue!(self.out, Print("  "))?;
                        }
                        Cell::Tail => {
                            queue!(self.out, SetBackgroundColor(Color::Red))?;
                            queue!(self.out, Print("  "))?;
                        }
                        Cell::Conductor => {
                            queue!(self.out, SetBackgroundColor(Color::Yellow))?;
                            queue!(self.out, Print("  "))?;
                        }
                    }
                    queue!(self.out, ResetColor)?;
                }
            }
        }
        self.out.flush()?;
        Ok(())
    }
}
