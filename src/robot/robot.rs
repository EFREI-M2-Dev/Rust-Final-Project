use rand::rngs::StdRng;
use rand::SeedableRng;

use super::{
    collector::Collector, explorator::Explorator, robot_type::RobotModule, robot_type::RobotType,
    scientist::Scientist,
};
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
    pub discovered_plan: Vec<(usize, usize)>,
    pub returning_to_base: bool,
    pub inventory: Vec<TileType>,
    pub max_capacity: usize,
    pub rng: StdRng,
    pub previous_positions: Vec<(usize, usize)>,
    pub modules: Vec<RobotModule>,
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
            discovered_plan: Vec::new(),
            returning_to_base: false,
            inventory: Vec::new(),
            max_capacity: 2,
            rng: StdRng::seed_from_u64(seed.into()),
            previous_positions: Vec::new(),
            modules: Vec::new(),
        }
    }

    pub fn add_module(&mut self, module: RobotModule) {
        self.modules.push(module);
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
            RobotType::Scientist => {
                Scientist::move_robot(self, grid, width, height, base);
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


#[cfg(test)]
mod tests {
    use super::*;

    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;
    const SEED: u32 = 42;

    fn create_grid() -> Vec<Vec<TileType>> {
        vec![vec![TileType::Empty; WIDTH]; HEIGHT]
    }

    // Test the creation of a Robot
    #[test]
    fn test_robot_creation_fields() {
        let robot = Robot::new(2, 3, RobotType::Collector, WIDTH, HEIGHT, SEED);
        assert_eq!(robot.x, 2);
        assert_eq!(robot.y, 3);
        assert_eq!(robot.base, (2, 3));
        assert_eq!(robot.visited_map.len(), HEIGHT);
        assert_eq!(robot.visited_map[0].len(), WIDTH);
        assert_eq!(robot.max_capacity, 2);
        assert!(robot.modules.is_empty());
        assert!(robot.inventory.is_empty());
    }

    // Test the creation of a Robot with a specific type
    #[test]
    fn test_add_module() {
        let mut robot = Robot::new(0, 0, RobotType::Collector, WIDTH, HEIGHT, SEED);
        robot.add_module(RobotModule::Sensor);
        assert_eq!(robot.modules.len(), 1);
        assert_eq!(robot.modules[0], RobotModule::Sensor);
    }
}