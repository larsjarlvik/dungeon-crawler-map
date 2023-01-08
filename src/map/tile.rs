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
pub struct Edges {
    pub north: Vec<u8>,
    pub east: Vec<u8>,
    pub south: Vec<u8>,
    pub west: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub edges: Edges,
    pub asset: usize,
    pub direction: Direction,
}

impl Hash for Tile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.edges.hash(state);
        self.direction.hash(state);
    }
}

pub fn get_edges(image: &image::DynamicImage) -> Edges {
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

    Edges { north, south, east, west }
}
