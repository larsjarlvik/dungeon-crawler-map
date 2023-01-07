use std::time::Instant;

use self::tile::{Direction, Tile};
use image::DynamicImage;
use rand::{rngs::ThreadRng, Rng};
mod tile;

#[derive(Debug, Default, Clone)]
pub struct Snapshot {
    pub grid: Vec<Option<Tile>>,
}

#[derive(Debug)]
pub struct Map {
    pub size: usize,
    pub history: Vec<Snapshot>,
    pub variants: Vec<Tile>,
}

impl Map {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            history: vec![],
            variants: vec![],
        }
    }

    pub fn load_tiles(&mut self, image: &mut DynamicImage, tile_size: u32) {
        let mut tiles = vec![];

        for x in 0..(image.width() / tile_size) {
            for y in 0..(image.height() / tile_size) {
                let varaint_img = image.crop(x * tile_size, y * tile_size, tile_size, tile_size);
                let mut variant = Tile {
                    edges: tile::get_edges(&varaint_img),
                    index: (y * self.size as u32 + x) as usize,
                    rotation: tile::Direction::North,
                    image: varaint_img,
                };

                // Rotate and store
                for _ in 0..4 {
                    tiles.push(variant.clone());
                    variant = variant.rotate();
                }
            }
        }

        self.variants = tiles;
        self.history.push(Snapshot {
            grid: self.variants.iter().map(|t| Some(t.clone())).collect(),
        });

        // Dedupe
        self.variants
            .dedup_by(|a, b| a.image.as_bytes() == b.image.as_bytes());
        self.history.push(Snapshot {
            grid: self.variants.iter().map(|t| Some(t.clone())).collect(),
        });
    }

    pub fn build(&mut self, rng: &mut ThreadRng) {
        let time = Instant::now();
        self.clear();

        let mut num_retries = 0;

        while !self.generate_map(rng) {
            num_retries += 1;
            self.history.clear();
        }

        let elapsed = time.elapsed().as_secs_f32();

        println!("Map generated after {} retries", num_retries);
        println!("Time taken: {}", elapsed);
        println!("Per try: {}", elapsed / (num_retries + 1) as f32);
    }

    fn generate_map(&mut self, rng: &mut ThreadRng) -> bool {
        self.clear();

        loop {
            let mut grid = self.history.last().cloned().unwrap().grid;
            let occupied = grid.iter().filter(|t| t.is_some()).count();
            if occupied == self.size * self.size {
                return true;
            }

            let free_neighbors = self.get_free_neighbors(&grid);
            let least_entropy = free_neighbors
                .iter()
                .min_by(|(_, a_tile), (_, b_tile)| a_tile.len().cmp(&b_tile.len()))
                .map(|(_, tile)| tile);

            if let Some(least_entropy) = least_entropy {
                let possibilties: Vec<&(usize, Vec<Tile>)> = free_neighbors
                    .iter()
                    .filter(|(index, _)| {
                        self.get_possible_variants(*index).len() == least_entropy.len()
                    })
                    .collect();

                if possibilties.is_empty() {
                    return false;
                }

                let (next_index, next_tile) = possibilties[rng.gen_range(0..possibilties.len())];
                if next_tile.is_empty() {
                    return false;
                }

                grid[*next_index] = Some(next_tile[rng.gen_range(0..next_tile.len())].clone());
                self.history.push(Snapshot { grid });
            } else {
                return false;
            }
        }
    }

    fn get_free_neighbors(&self, grid: &[Option<Tile>]) -> Vec<(usize, Vec<Tile>)> {
        let mut neighbors = vec![];

        for index in 0..grid.len() {
            let directions = vec![
                self.move_index(index, Direction::North),
                self.move_index(index, Direction::East),
                self.move_index(index, Direction::South),
                self.move_index(index, Direction::West),
            ];

            let mut possibilities: Vec<(usize, Vec<Tile>)> = directions
                .into_iter()
                .flatten()
                .filter(|index| grid[*index].is_none())
                .map(|index| (index, self.get_possible_variants(index)))
                .collect();

            neighbors.append(&mut possibilities);
        }

        neighbors
    }

    fn clear(&mut self) {
        let mut grid: Vec<Option<Tile>> = vec![];
        for _ in 0..self.size {
            for _ in 0..self.size {
                grid.push(None)
            }
        }

        self.history.push(Snapshot { grid });
    }

    fn get_possible_variants(&self, index: usize) -> Vec<Tile> {
        let variants = self
            .variants
            .iter()
            .filter(|variant| {
                if let Some(index) = self.move_index(index, Direction::North) {
                    if let Some(tile) = self.get_tile(index) {
                        if variant.edges.north != tile.edges.south {
                            return false;
                        }
                    }
                }

                if let Some(index) = self.move_index(index, Direction::East) {
                    if let Some(tile) = self.get_tile(index) {
                        if variant.edges.east != tile.edges.west {
                            return false;
                        }
                    }
                }

                if let Some(index) = self.move_index(index, Direction::South) {
                    if let Some(tile) = self.get_tile(index) {
                        if variant.edges.south != tile.edges.north {
                            return false;
                        }
                    }
                }

                if let Some(index) = self.move_index(index, Direction::West) {
                    if let Some(tile) = self.get_tile(index) {
                        if variant.edges.west != tile.edges.east {
                            return false;
                        }
                    }
                }

                true
            })
            .cloned()
            .collect();

        variants
    }

    fn move_index(&self, current: usize, direction: Direction) -> Option<usize> {
        match direction {
            Direction::North => {
                if current > self.size {
                    Some(current - self.size)
                } else {
                    None
                }
            }
            Direction::East => {
                if current % self.size + 1 < self.size {
                    Some(current + 1)
                } else {
                    None
                }
            }
            Direction::South => {
                if current + self.size < self.size * self.size {
                    Some(current + self.size)
                } else {
                    None
                }
            }
            Direction::West => {
                if current % self.size > 0 {
                    Some(current - 1)
                } else {
                    None
                }
            }
        }
    }

    fn get_tile(&self, index: usize) -> Option<&Tile> {
        let map = self.history.last().expect("No history!");

        if let Some(tile) = map.grid.get(index) {
            tile.into()
        } else {
            None
        }
    }
}
