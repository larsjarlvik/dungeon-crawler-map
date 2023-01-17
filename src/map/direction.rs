#[derive(Debug, Clone, Hash)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

pub fn move_index(current: usize, size: usize, direction: Direction) -> Option<usize> {
    match direction {
        Direction::North => {
            if current > size {
                return Some(current - size);
            }
        }
        Direction::East => {
            if current % size + 1 < size {
                return Some(current + 1);
            }
        }
        Direction::South => {
            if current + size < size * size {
                return Some(current + size);
            }
        }
        Direction::West => {
            if current % size > 0 {
                return Some(current - 1);
            }
        }
    }

    None
}
