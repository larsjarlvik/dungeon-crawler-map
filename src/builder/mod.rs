use self::tile::TileType;
pub mod tile;
mod util;
mod wfc;

pub const MAPWIDTH: usize = 63;
pub const MAPHEIGHT: usize = 63;
pub const MAPCOUNT: usize = MAPHEIGHT * MAPWIDTH;

#[derive(Default, Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Empty; MAPCOUNT],
            width: MAPWIDTH as i32,
            height: MAPHEIGHT as i32,
        }
    }

    pub fn build(map: Map) -> Vec<Self> {
        let mut wfc = wfc::Wfc::new(map); // TODO: Check value
        wfc.build();
        wfc.history
    }

    pub fn from_string(string: String) -> Self {
        let lines = string.split("\r\n").collect::<Vec<&str>>();
        let width = lines[0].chars().count() as i32;
        let height = lines.len() as i32;
        let mut tiles = vec![];

        for line in lines.iter() {
            for char in line.chars() {
                match char {
                    '┌' => tiles.push(TileType::CornerTL),
                    '┐' => tiles.push(TileType::CornerTR),
                    '┘' => tiles.push(TileType::CornerBR),
                    '└' => tiles.push(TileType::CornerBL),
                    '│' => tiles.push(TileType::WallL),
                    '─' => tiles.push(TileType::WallT),
                    '┆' => tiles.push(TileType::WallR),
                    '┄' => tiles.push(TileType::WallB),
                    ' ' => tiles.push(TileType::Floor),
                    _ => {}
                }
            }
        }

        Self {
            tiles,
            width,
            height,
        }
    }

    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }
}
