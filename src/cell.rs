#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cell {
    Empty,
    Head,
    Tail,
    Conductor,
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Empty
    }
}

impl Cell {
    /// `neighbors` — количество соседей в состоянии Head (0..=8).
    pub fn step(&mut self, neighbors: u8) {
        *self = match *self {
            Cell::Empty => Cell::Empty,
            Cell::Head => Cell::Tail,
            Cell::Tail => Cell::Conductor,
            Cell::Conductor => {
                if neighbors == 1 || neighbors == 2 {
                    Cell::Head
                } else {
                    Cell::Conductor
                }
            }
        };
    }
}
