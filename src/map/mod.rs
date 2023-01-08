use self::tile::{Direction, Tile, TileEdges};
use image::DynamicImage;
use itertools::Itertools;
use rand::{rngs::ThreadRng, Rng};
use std::time::Instant;
mod tile;

#[derive(Debug, Clone)]
pub struct GridTile {
    pub edges: TileEdges,
    pub index: usize,
    pub rotation: Direction,
}

impl GridTile {
    pub fn from_variant(tile: &Tile) -> Self {
        Self {
            edges: tile.edges.clone(),
            index: tile.index,
            rotation: tile.rotation.clone(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Snapshot {
    pub grid: Vec<Option<GridTile>>,
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
            grid: self
                .variants
                .iter()
                .map(|t| Some(GridTile::from_variant(t)))
                .collect(),
        });

        // Dedupe
        self.variants
            .dedup_by(|a, b| a.image.as_bytes() == b.image.as_bytes());
        self.history.push(Snapshot {
            grid: self
                .variants
                .iter()
                .map(|t| Some(GridTile::from_variant(t)))
                .collect(),
        });
    }

    pub fn build(&mut self, rng: &mut ThreadRng, log_history: bool) {
        let time = Instant::now();

        let mut tries = 1;
        while !self.generate_map(rng, log_history) {
            tries += 1;
            self.history.clear();
        }

        let elapsed = time.elapsed().as_secs_f32();

        println!("Map generated after {} tries", tries);
        println!("Time taken: {}", elapsed);
        println!("Per try: {}", elapsed / tries as f32);
    }

    fn generate_map(&mut self, rng: &mut ThreadRng, step_by_step: bool) -> bool {
        let mut grid = self.clear(rng);
        if step_by_step {
            self.history.push(Snapshot { grid: grid.clone() });
        }

        loop {
            if grid.iter().all(|tile| tile.is_some()) {
                self.history.push(Snapshot { grid });
                return true;
            }

            let free_neighbors = self.get_free_neighbors(&grid);
            let least_entropy = free_neighbors
                .iter()
                .min_by(|(_, a_tile), (_, b_tile)| a_tile.len().cmp(&b_tile.len()))
                .map(|(_, tile)| tile);

            if let Some(least_entropy) = least_entropy {
                let possibilties: Vec<&(usize, Vec<usize>)> = free_neighbors
                    .iter()
                    .filter(|(index, _)| {
                        self.get_possible_variants(&grid, *index).len() == least_entropy.len()
                    })
                    .collect();

                let (next_index, next_tile) = possibilties[rng.gen_range(0..possibilties.len())];
                if next_tile.is_empty() {
                    return false;
                }

                let variant = next_tile[rng.gen_range(0..next_tile.len())];
                grid[*next_index] = Some(GridTile::from_variant(&self.variants[variant]));

                if step_by_step {
                    self.history.push(Snapshot { grid: grid.clone() });
                }
            } else {
                return false;
            }
        }
    }

    fn get_free_neighbors(&self, grid: &[Option<GridTile>]) -> Vec<(usize, Vec<usize>)> {
        (0..grid.len())
            .into_iter()
            .filter(|index| grid[*index].is_some())
            .flat_map(|index| {
                let neighbors = [
                    self.move_index(index, Direction::North),
                    self.move_index(index, Direction::East),
                    self.move_index(index, Direction::South),
                    self.move_index(index, Direction::West),
                ];

                neighbors.into_iter().flatten().filter_map(|index| {
                    if grid[index].is_none() {
                        Some((index, self.get_possible_variants(grid, index)))
                    } else {
                        None
                    }
                })
            })
            .unique()
            .collect()
    }

    fn clear(&mut self, rng: &mut ThreadRng) -> Vec<Option<GridTile>> {
        let mut grid = vec![None; self.size * self.size];
        grid[rng.gen_range(0..(self.size * self.size))] = Some(GridTile::from_variant(
            &self.variants[rng.gen_range(0..self.variants.len())],
        ));
        grid
    }

    fn get_possible_variants(&self, grid: &[Option<GridTile>], index: usize) -> Vec<usize> {
        let variants = self
            .variants
            .iter()
            .enumerate()
            .filter_map(|(variant_index, variant)| {
                if let Some(tile) = self.get_tile(grid, self.move_index(index, Direction::North)) {
                    if variant.edges.north != tile.edges.south {
                        return None;
                    }
                }

                if let Some(tile) = self.get_tile(grid, self.move_index(index, Direction::East)) {
                    if variant.edges.east != tile.edges.west {
                        return None;
                    }
                }

                if let Some(tile) = self.get_tile(grid, self.move_index(index, Direction::South)) {
                    if variant.edges.south != tile.edges.north {
                        return None;
                    }
                }

                if let Some(tile) = self.get_tile(grid, self.move_index(index, Direction::West)) {
                    if variant.edges.west != tile.edges.east {
                        return None;
                    }
                }

                Some(variant_index)
            })
            .collect();

        variants
    }

    fn move_index(&self, current: usize, direction: Direction) -> Option<usize> {
        match direction {
            Direction::North => {
                if current > self.size {
                    return Some(current - self.size);
                }
            }
            Direction::East => {
                if current % self.size + 1 < self.size {
                    return Some(current + 1);
                }
            }
            Direction::South => {
                if current + self.size < self.size * self.size {
                    return Some(current + self.size);
                }
            }
            Direction::West => {
                if current % self.size > 0 {
                    return Some(current - 1);
                }
            }
        }

        None
    }

    fn get_tile<'a>(
        &self,
        grid: &'a [Option<GridTile>],
        index: Option<usize>,
    ) -> Option<&'a GridTile> {
        if let Some(index) = index {
            if let Some(tile) = grid.get(index) {
                tile.as_ref()
            } else {
                None
            }
        } else {
            None
        }
    }
}
