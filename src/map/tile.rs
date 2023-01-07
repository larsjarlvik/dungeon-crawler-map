use image::GenericImageView;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Hash)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

#[derive(Debug, Clone, Hash)]
pub struct TileEdges {
    pub north: Vec<u8>,
    pub east: Vec<u8>,
    pub south: Vec<u8>,
    pub west: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub edges: TileEdges,
    pub index: usize,
    pub rotation: Direction,
    pub image: image::DynamicImage,
}

impl Tile {
    pub fn rotate(&self) -> Self {
        let mut tile = self.clone();
        tile.edges.north = self.edges.west.clone();
        tile.edges.east = self.edges.north.clone();
        tile.edges.south = self.edges.east.clone();
        tile.edges.west = self.edges.south.clone();
        tile.rotation = match self.rotation {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        tile.image = tile.image.rotate90();
        tile
    }
}

impl Hash for Tile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.edges.hash(state);
        self.rotation.hash(state);
    }
}

pub fn get_edges(image: &image::DynamicImage) -> TileEdges {
    let mut north = vec![];
    let mut south = vec![];
    let mut east = vec![];
    let mut west = vec![];

    for x in 0..image.width() {
        north.push(*image.get_pixel(x, 0).0.first().unwrap());
        south.push(*image.get_pixel(x, image.height() - 1).0.first().unwrap());
    }

    for y in 0..image.height() {
        west.push(*image.get_pixel(0, y).0.first().unwrap());
        east.push(*image.get_pixel(image.width() - 1, y).0.first().unwrap());
    }

    TileEdges {
        north,
        south,
        east,
        west,
    }
}
