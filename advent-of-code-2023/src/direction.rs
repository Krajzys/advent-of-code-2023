#[derive(PartialEq, Clone, Copy, Eq, Hash, Debug)]
pub enum Direction {
    Top,
    Right,
    Bottom,
    Left
}

impl Direction {
    pub fn get_opposite(&self) -> Direction {
        match self {
            Direction::Bottom => Direction::Top,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Top => Direction::Bottom
        }
    }

    pub fn move_position(&self, position: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Bottom => (position.0 + 1, position.1),
            Direction::Left => (position.0, position.1 - 1),
            Direction::Right => (position.0, position.1 + 1),
            Direction::Top => (position.0 - 1, position.1)
        }
    }
}