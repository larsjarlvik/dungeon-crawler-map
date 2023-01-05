use self::common::MapChunk;
use super::{tile, Map, TileType};
use crate::builder::wfc::{common::patterns_to_constraints, solver::Solver};
use std::collections::HashSet;
mod common;
mod solver;
use rand::thread_rng;

pub struct Wfc {
    map: Map,
    pub history: Vec<Map>,
}

impl Wfc {
    pub fn new(map: Map) -> Wfc {
        Wfc {
            map,
            history: Vec::new(),
        }
    }

    fn take_snapshot(&mut self) {
        self.history.push(self.map.clone());
    }

    pub fn build(&mut self) {
        let mut rng = thread_rng();

        self.take_snapshot();

        const CHUNK_SIZE: i32 = 5;

        let patterns = build_patterns(&self.map, CHUNK_SIZE, true, true);
        let constraints = patterns_to_constraints(patterns, CHUNK_SIZE);
        self.render_tile_gallery(&constraints, CHUNK_SIZE);

        self.map = Map::new();
        loop {
            let mut solver = Solver::new(constraints.clone(), CHUNK_SIZE, &self.map);
            while !solver.iteration(&mut self.map, &mut rng) {
                self.take_snapshot();
            }
            self.take_snapshot();
            if solver.possible {
                break;
            }
        }
    }

    fn render_tile_gallery(&mut self, constraints: &Vec<MapChunk>, chunk_size: i32) {
        self.map = Map::new();
        let mut counter = 0;
        let mut x = 1;
        let mut y = 1;
        while counter < constraints.len() {
            render_pattern_to_map(&mut self.map, &constraints[counter], chunk_size, x, y);

            x += chunk_size + 1;
            if x + chunk_size > self.map.width {
                // Move to the next row
                x = 1;
                y += chunk_size + 1;

                if y + chunk_size > self.map.height {
                    // Move to the next page
                    self.take_snapshot();
                    self.map = Map::new();

                    x = 1;
                    y = 1;
                }
            }

            counter += 1;
        }

        self.take_snapshot();
    }
}

pub fn build_patterns(
    map: &Map,
    chunk_size: i32,
    include_flipping: bool,
    dedupe: bool,
) -> Vec<Vec<TileType>> {
    let chunks_x = map.width / chunk_size;
    let chunks_y = map.height / chunk_size;
    let mut patterns = Vec::new();

    for cy in 0..chunks_y {
        for cx in 0..chunks_x {
            // Normal orientation
            let mut pattern: Vec<TileType> = Vec::new();
            let start_x = cx * chunk_size;
            let end_x = (cx + 1) * chunk_size;
            let start_y = cy * chunk_size;
            let end_y = (cy + 1) * chunk_size;

            for y in start_y..end_y {
                for x in start_x..end_x {
                    let idx = map.xy_idx(x, y);
                    pattern.push(map.tiles[idx]);
                }
            }
            patterns.push(pattern);

            if include_flipping {
                // Flip horizontal
                pattern = Vec::new();
                for y in start_y..end_y {
                    for x in start_x..end_x {
                        let idx = map.xy_idx(end_x - (x + 1), y);
                        pattern.push(tile::flip_x(map.tiles[idx]));
                    }
                }
                patterns.push(pattern);

                // Flip vertical
                pattern = Vec::new();
                for y in start_y..end_y {
                    for x in start_x..end_x {
                        let idx = map.xy_idx(x, end_y - (y + 1));
                        pattern.push(tile::flip_y(map.tiles[idx]));
                    }
                }
                patterns.push(pattern);

                // Flip both
                pattern = Vec::new();
                for y in start_y..end_y {
                    for x in start_x..end_x {
                        let idx = map.xy_idx(end_x - (x + 1), end_y - (y + 1));
                        pattern.push(tile::flip_x(tile::flip_y(map.tiles[idx])));
                    }
                }
                patterns.push(pattern);
            }
        }
    }

    // Dedupe
    if dedupe {
        println!("Pre de-duplication, there are {} patterns", patterns.len());
        let set: HashSet<Vec<TileType>> = patterns.drain(..).collect(); // dedup
        patterns.extend(set.into_iter());
        println!("There are {} patterns", patterns.len());
    }

    patterns
}

pub fn render_pattern_to_map(
    map: &mut Map,
    chunk: &MapChunk,
    chunk_size: i32,
    start_x: i32,
    start_y: i32,
) {
    let mut i = 0usize;
    for tile_y in 0..chunk_size {
        for tile_x in 0..chunk_size {
            let map_idx = map.xy_idx(start_x + tile_x, start_y + tile_y);
            map.tiles[map_idx] = chunk.pattern[i];
            i += 1;
        }
    }

    for (x, northbound) in chunk.exits[0].iter().enumerate() {
        if *northbound {
            let map_idx = map.xy_idx(start_x + x as i32, start_y);
            map.tiles[map_idx] = TileType::Constraint;
        }
    }
    for (x, southbound) in chunk.exits[1].iter().enumerate() {
        if *southbound {
            let map_idx = map.xy_idx(start_x + x as i32, start_y + chunk_size - 1);
            map.tiles[map_idx] = TileType::Constraint;
        }
    }
    for (x, westbound) in chunk.exits[2].iter().enumerate() {
        if *westbound {
            let map_idx = map.xy_idx(start_x, start_y + x as i32);
            map.tiles[map_idx] = TileType::Constraint;
        }
    }
    for (x, eastbound) in chunk.exits[3].iter().enumerate() {
        if *eastbound {
            let map_idx = map.xy_idx(start_x + chunk_size - 1, start_y + x as i32);
            map.tiles[map_idx] = TileType::Constraint;
        }
    }
}
