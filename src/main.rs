mod cell;
mod grid;
mod renderer;

use std::time::Duration;
use crossterm::{
    execute, cursor,
    event::{self, Event, KeyCode},
    terminal::{
        enable_raw_mode, disable_raw_mode,
        EnterAlternateScreen, LeaveAlternateScreen
    },
};
use cell::Cell;
use grid::Grid;
use renderer::Renderer;

fn main() -> std::io::Result<()> {
    let mut grid = Grid::<Cell>::new(80, 24);
    let mut renderer = Renderer::new();

    enable_raw_mode()?;
    execute!(renderer.out, EnterAlternateScreen)?;

    let result = run_app(&mut renderer, &mut grid);

    execute!(renderer.out, LeaveAlternateScreen)?;
    disable_raw_mode()?;

    result
}

fn run_app(renderer: &mut Renderer, grid: &mut Grid<Cell>) -> std::io::Result<()> {
    let mut cx = 0i32;
    let mut cy = 0i32;
    let mut pause = true;
    loop {
        if !pause {
            let snapshot = grid.clone();
            for y in 0..grid.h as i32 {
                for x in 0..grid.w as i32 {
                    let neighbors = snapshot.count_neighbors(x, y);
                    if let Some(cell) = grid.get_mut(x, y) {
                        cell.step(neighbors);
                    }
                }
            }
        }

        if event::poll(Duration::from_millis(500))? {
            let ev = event::read()?;
            match ev {
                Event::Key(key) => match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('h') => cx = (cx - 1).clamp(0, grid.w as i32 - 1),
                    KeyCode::Char('j') => cy = (cy + 1).clamp(0, grid.h as i32 - 1),
                    KeyCode::Char('k') => cy = (cy - 1).clamp(0, grid.h as i32 - 1),
                    KeyCode::Char('l') => cx = (cx + 1).clamp(0, grid.w as i32 - 1),
                    KeyCode::Char('.') => {
                        if let Some(cell) = grid.get_mut(cx, cy) {
                            *cell = Cell::Empty;
                        }
                    }
                    KeyCode::Char('c') => {
                        if let Some(cell) = grid.get_mut(cx, cy) {
                            *cell = Cell::Conductor;
                        }
                    }
                    KeyCode::Char('v') => {
                        if let Some(cell) = grid.get_mut(cx, cy) {
                            *cell = Cell::Head;
                        }
                    }
                    KeyCode::Char(' ') => if pause {
                        pause = false;
                    } else {
                        pause = true;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        renderer.draw(&grid)?;
        execute!(renderer.out, cursor::MoveTo(cx as u16, cy as u16))?;
    }
}
