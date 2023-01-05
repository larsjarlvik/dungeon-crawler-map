pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn zero() -> Self {
        Position { x: 0, y: 0 }
    }
}
