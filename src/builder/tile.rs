#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum TileType {
    Empty,
    Floor,
    Constraint,
    CornerTL,
    CornerTR,
    CornerBR,
    CornerBL,
    WallL,
    WallT,
    WallR,
    WallB,
}

pub fn flip_x(tile_type: TileType) -> TileType {
    match tile_type {
        TileType::Empty => TileType::Empty,
        TileType::Floor => TileType::Floor,
        TileType::Constraint => TileType::Constraint,
        TileType::CornerTL => TileType::CornerTR,
        TileType::CornerTR => TileType::CornerTL,
        TileType::CornerBR => TileType::CornerBL,
        TileType::CornerBL => TileType::CornerBR,
        TileType::WallL => TileType::WallR,
        TileType::WallT => TileType::WallT,
        TileType::WallR => TileType::WallL,
        TileType::WallB => TileType::WallB,
    }
}

pub fn flip_y(tile_type: TileType) -> TileType {
    match tile_type {
        TileType::Empty => TileType::Empty,
        TileType::Floor => TileType::Floor,
        TileType::Constraint => TileType::Constraint,
        TileType::CornerTL => TileType::CornerBL,
        TileType::CornerTR => TileType::CornerBR,
        TileType::CornerBR => TileType::CornerTR,
        TileType::CornerBL => TileType::CornerTL,
        TileType::WallL => TileType::WallL,
        TileType::WallT => TileType::WallB,
        TileType::WallR => TileType::WallR,
        TileType::WallB => TileType::WallT,
    }
}
