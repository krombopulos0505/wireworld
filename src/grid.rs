use crate::cell::Cell;

#[derive(Clone)]
pub struct Grid<T> {
    pub w: usize,
    pub h: usize,
    cells: Vec<T>,
}

impl<T: Default + Clone> Grid<T> {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            w,
            h,
            cells: vec![T::default(); w * h],
        }
    }
}

impl<T> Grid<T> {
    fn index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 || x as usize >= self.w || y as usize >= self.h {
            return None;
        }
        Some(y as usize * self.w + x as usize)
    }

    pub fn get(&self, x: i32, y: i32) -> Option<&T> {
        self.index(x, y).map(|i| &self.cells[i])
    }

    pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut T> {
        self.index(x, y).map(move |i| &mut self.cells[i])
    }
}

impl Grid<Cell> {
    pub fn count_neighbors(&self, x: i32, y: i32) -> u8 {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if let Some(Cell::Head) = self.get(x + dx, y + dy) {
                    count += 1;
                }
            }
        }
        count
    }
}
