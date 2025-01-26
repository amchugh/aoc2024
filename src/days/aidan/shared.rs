
type PositionT = u32;
pub type Position = (PositionT, PositionT);

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right
}

impl Direction {
    pub fn clockwise(&self) -> Direction {
        match &self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    pub fn next(&self, position: &Position) -> Option<Position> {
        match self {
            Direction::Up => Some((position.0, position.1.checked_sub(1)?)),
            Direction::Left => Some((position.0.checked_sub(1)?, position.1)),
            Direction::Down => Some((position.0, position.1.checked_add(1)?)),
            Direction::Right => Some((position.0.checked_add(1)?, position.1)),
        }
    }
}

#[derive(Clone)]
pub struct Grid<T> {
    grid: Vec<Vec<T>>, 
    width: PositionT
}

impl<T> Grid<T> {
    pub fn from(grid: Vec<Vec<T>>) -> Self {
        let width = grid.first().map_or(0, |row| row.len().try_into().unwrap());
        Grid{grid, width}
    }

    pub fn get(&self, position: &Position) -> Option<&T> {
        let (x, y) = *position;
        if x >= self.width || y >= self.grid.len() as PositionT {
            None
        } else {
            Some(&self.grid[y as usize][x as usize])
        }
    }

    pub fn set(&mut self, position: &Position, value: T) -> Result<(), ()> {
        let (x, y) = *position;
        if x >= self.width || y >= self.grid.len() as PositionT {
            Err(())
        } else {
            self.grid[y as usize][x as usize] = value;
            Ok(())
        }
    }
}

impl<T> Default for Grid<T> {
    fn default() -> Self {
        Grid {
            grid: Vec::new(),
            width: 0,
        }
    }
}
