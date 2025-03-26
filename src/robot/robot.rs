use rand::rngs::StdRng;
use rand::SeedableRng;

use super::collector::Collector;
use super::explorator::Explorator;
use super::robot_type::RobotType;
use crate::map::{base::Base, TileType};

#[derive(Debug)]
pub struct Robot {
    pub x: usize,
    pub y: usize,
    pub robot_type: RobotType,
    pub base: (usize, usize),
    pub target: Option<(usize, usize)>,
    pub visited_map: Vec<Vec<bool>>,
    pub discovered_minerals: Vec<(usize, usize)>,
    pub discovered_energy: Vec<(usize, usize)>,
    pub returning_to_base: bool,
    pub inventory: Vec<TileType>,
    pub max_capacity: usize,
    pub rng: StdRng,
    pub previous_positions: Vec<(usize, usize)>,
}

impl Robot {
    pub fn new(
        x: usize,
        y: usize,
        robot_type: RobotType,
        width: usize,
        height: usize,
        seed: u32,
    ) -> Self {
        Robot {
            x,
            y,
            base: (x, y),
            robot_type,
            target: None,
            visited_map: vec![vec![false; width]; height],
            discovered_minerals: Vec::new(),
            discovered_energy: Vec::new(),
            returning_to_base: false,
            inventory: Vec::new(),
            max_capacity: 2,
            rng: StdRng::seed_from_u64(seed.into()),
            previous_positions: Vec::new(),
        }
    }

    pub fn move_robot(
        &mut self,
        grid: &mut Vec<Vec<TileType>>,
        width: usize,
        height: usize,
        base: &mut Base,
    ) {
        match self.robot_type {
            RobotType::Explorator => {
                Explorator::move_robot(self, grid, width, height, base);
            }
            RobotType::Collector => {
                Collector::move_robot(self, grid, width, height, base);
            }
        }
    }

    pub fn move_towards(
        &mut self,
        tx: usize,
        ty: usize,
        grid: &mut Vec<Vec<TileType>>,
        width: usize,
        height: usize,
    ) {
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        let mut best_x = self.x;
        let mut best_y = self.y;
        let mut min_distance = usize::MAX;
        let mut found_new_tile = false;

        for (dx, dy) in directions.iter() {
            let nx = self.x as isize + dx;
            let ny = self.y as isize + dy;

            if nx >= 0 && ny >= 0 && nx < width as isize && ny < height as isize {
                let nx = nx as usize;
                let ny = ny as usize;

                let distance = (nx as isize - tx as isize).abs() as usize
                    + (ny as isize - ty as isize).abs() as usize;

                if grid[ny][nx] == TileType::Empty && distance < min_distance {
                    if !self.previous_positions.contains(&(nx, ny)) {
                        best_x = nx;
                        best_y = ny;
                        min_distance = distance;
                        found_new_tile = true;
                    }
                }
            }
        }

        self.x = best_x;
        self.y = best_y;

        self.previous_positions.push((self.x, self.y));

        if self.previous_positions.len() > 5 {
            self.previous_positions.remove(0);
        }
    }
}
