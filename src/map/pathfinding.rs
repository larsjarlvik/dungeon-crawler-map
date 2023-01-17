use super::{
    direction::{self, Direction},
    Tile,
};
use pathfinding::prelude::astar;

pub struct Pathfinding {
    size: usize,
    grid: Vec<Tile>,
}

impl Pathfinding {
    pub fn new(size: usize, grid: Vec<Option<Tile>>) -> Self {
        Self {
            size,
            grid: grid.into_iter().flatten().collect(),
        }
    }

    fn get_successors(&self, position: usize) -> Vec<usize> {
        let mut successors = Vec::new();

        if let Some(tile) = self.grid.get(position) {
            if tile.edges.north.iter().any(|e| e > &0) {
                successors.push(direction::move_index(position, self.size, Direction::North).unwrap());
            }
            if tile.edges.east.iter().any(|e| e > &0) {
                successors.push(direction::move_index(position, self.size, Direction::East).unwrap());
            }
            if tile.edges.south.iter().any(|e| e > &0) {
                successors.push(direction::move_index(position, self.size, Direction::South).unwrap());
            }
            if tile.edges.west.iter().any(|e| e > &0) {
                successors.push(direction::move_index(position, self.size, Direction::West).unwrap());
            }
        }

        successors
    }

    fn distance(&self, position: usize, goal: usize) -> usize {
        let position = (position % self.size, position / self.size);
        let goal = (goal % self.size, goal / self.size);

        ((position.0 as i32 - goal.0 as i32).abs() + (position.1 as i32 - goal.1 as i32).abs()) as usize
    }

    pub fn test(&self, start: (usize, usize), goal: (usize, usize)) -> Option<(Vec<usize>, usize)> {
        let start = start.1 * self.size + start.0;
        let goal = goal.1 * self.size + goal.0;

        let result = astar(
            &start,
            |p| self.get_successors(*p).iter().map(|s| (*s, 1)).collect::<Vec<_>>(),
            |p| self.distance(*p, goal),
            |p| *p == goal,
        );

        result
    }
}
