use image::DynamicImage;
use itertools::Itertools;
use rand::distributions::WeightedIndex;
use rand::prelude::Distribution;
use rand::{rngs::ThreadRng, Rng};
use std::time::Instant;
mod tile;
pub use tile::Direction;
pub use tile::Edges;
pub use tile::Tile;

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

pub struct Variants {
    pub index: usize,
    pub weight: f32,
    pub neighbors: Vec<usize>,
}

pub struct Config {
    pub image: Option<(DynamicImage, u32)>,
    pub variants: Vec<Variants>,
}

impl Map {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            history: vec![],
            variants: vec![],
        }
    }

    pub fn build(&mut self, rng: &mut ThreadRng, config: &Config, log_history: bool) {
        self.history.clear();
        let time = Instant::now();

        self.load_config(config);
        self.history.push(Snapshot {
            grid: self.variants.iter().map(|v| Some(v.clone())).collect(),
        });

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

    fn neighbors_from_image(&self, image: &DynamicImage, tile_size: u32) -> Vec<(usize, Direction, Edges)> {
        let mut variants = vec![];

        for x in 0..(image.width() / tile_size) {
            for y in 0..(image.height() / tile_size) {
                let index = (y * self.size as u32 + x) as usize;
                let mut varaint_img = image.clone().crop(x * tile_size, y * tile_size, tile_size, tile_size);
                let mut direction = Direction::North;

                for _ in 0..4 {
                    variants.push((index, direction.clone(), varaint_img.clone()));
                    varaint_img = varaint_img.rotate90();
                    direction = match direction {
                        Direction::North => Direction::East,
                        Direction::East => Direction::South,
                        Direction::South => Direction::West,
                        Direction::West => Direction::North,
                    };
                }
            }
        }

        variants.sort_by(|(_, _, a), (_, _, b)| a.as_bytes().cmp(b.as_bytes()));
        variants.dedup_by(|(_, _, a), (_, _, b)| a.as_bytes() == b.as_bytes());

        variants
            .into_iter()
            .map(|(index, direction, image)| (index, direction, tile::get_edges(&image)))
            .collect()
    }

    fn load_config(&mut self, config: &Config) {
        self.variants.clear();

        let neighbors = if let Some((image, tile_size)) = &config.image {
            self.neighbors_from_image(image, *tile_size)
        } else {
            vec![]
        };

        for (asset, direction, edges) in neighbors {
            self.variants.push(Tile {
                asset,
                direction,
                edges,
                weight: 1.0,
                neighbors: vec![],
            });
        }

        for variant in config.variants.iter() {
            if let Some(existing) = self.variants.iter_mut().find(|v| v.asset == variant.index) {
                existing.weight = variant.weight;
                existing.neighbors = variant.neighbors.clone();
            } else {
                self.variants.push(Tile {
                    asset: variant.index,
                    direction: Direction::North,
                    edges: Edges::default(),
                    neighbors: variant.neighbors.clone(),
                    weight: variant.weight,
                })
            }
        }

        if self.variants.is_empty() {
            panic!("No variants set for map!");
        }
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
                .min_by(|(_, a_tile), (_, b_tile)| {
                    let a_sum: f32 = a_tile.iter().map(|a| self.variants[*a].weight).sum();
                    let b_sum: f32 = b_tile.iter().map(|b| self.variants[*b].weight).sum();
                    a_sum.partial_cmp(&b_sum).unwrap()
                })
                .map(|(_, tile)| tile);

            if let Some(least_entropy) = least_entropy {
                let possibilties: Vec<&(usize, Vec<usize>)> = free_neighbors
                    .iter()
                    .filter(|(index, _)| self.get_possible_variants(&grid, *index).len() == least_entropy.len())
                    .collect();

                let (next_index, next_tile) = possibilties[rng.gen_range(0..possibilties.len())];
                if next_tile.is_empty() {
                    return false;
                }

                grid[*next_index] = Some(self.weighted_variant(rng, next_tile));

                if step_by_step {
                    self.history.push(Snapshot { grid: grid.clone() });
                }
            } else {
                return false;
            }
        }
    }

    fn weighted_variant(&self, rng: &mut ThreadRng, variants: &[usize]) -> Tile {
        let weights: Vec<f32> = variants.iter().map(|v| self.variants[*v].weight).collect();
        let dist = WeightedIndex::new(&weights).unwrap();

        self.variants[variants[dist.sample(rng)]].clone()
    }

    fn get_free_neighbors(&self, grid: &[Option<Tile>]) -> Vec<(usize, Vec<usize>)> {
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

    fn clear(&mut self, rng: &mut ThreadRng) -> Vec<Option<Tile>> {
        let mut grid = vec![None; self.size * self.size];
        grid[rng.gen_range(0..(self.size * self.size))] = Some(self.variants[rng.gen_range(0..self.variants.len())].clone());
        grid
    }

    fn get_possible_variants(&self, grid: &[Option<Tile>], index: usize) -> Vec<usize> {
        let variants = self
            .variants
            .iter()
            .enumerate()
            .filter_map(|(variant_index, variant)| {
                if let Some(tile) = self.get_tile(grid, self.move_index(index, Direction::North)) {
                    if variant.edges.north != tile.edges.south && !variant.neighbors.contains(&tile.asset) {
                        return None;
                    }
                }

                if let Some(tile) = self.get_tile(grid, self.move_index(index, Direction::East)) {
                    if variant.edges.east != tile.edges.west && !variant.neighbors.contains(&tile.asset) {
                        return None;
                    }
                }

                if let Some(tile) = self.get_tile(grid, self.move_index(index, Direction::South)) {
                    if variant.edges.south != tile.edges.north && !variant.neighbors.contains(&tile.asset) {
                        return None;
                    }
                }

                if let Some(tile) = self.get_tile(grid, self.move_index(index, Direction::West)) {
                    if variant.edges.west != tile.edges.east && !variant.neighbors.contains(&tile.asset) {
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

    fn get_tile<'a>(&self, grid: &'a [Option<Tile>], index: Option<usize>) -> Option<&'a Tile> {
        match index {
            Some(index) => match grid.get(index) {
                Some(tile) => tile.as_ref(),
                None => None,
            },
            None => None,
        }
    }
}
