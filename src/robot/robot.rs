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
    use rand::Rng;

    const WIDTH: usize = 10;
    const HEIGHT: usize = 10;
    const SEED: u32 = 42;

    fn create_grid() -> Vec<Vec<TileType>> {
        vec![vec![TileType::Empty; WIDTH]; HEIGHT]
    }

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

    #[test]
    fn test_add_module() {
        let mut robot = Robot::new(0, 0, RobotType::Collector, WIDTH, HEIGHT, SEED);
        robot.add_module(RobotModule::Sensor);
        assert_eq!(robot.modules.len(), 1);
        assert_eq!(robot.modules[0], RobotModule::Sensor);
    }

    #[test]
    fn test_move_towards_decreases_distance() {
        let mut robot = Robot::new(5, 5, RobotType::Collector, WIDTH, HEIGHT, SEED);
        let mut grid = create_grid();
        let target = (7, 5);
        let old_distance = ((robot.x as isize - target.0 as isize).abs()
            + (robot.y as isize - target.1 as isize).abs()) as usize;
        robot.move_towards(target.0, target.1, &mut grid, WIDTH, HEIGHT);
        let new_distance = ((robot.x as isize - target.0 as isize).abs()
            + (robot.y as isize - target.1 as isize).abs()) as usize;
        assert!(new_distance < old_distance);
    }

    #[test]
    fn test_move_towards_updates_previous_positions() {
        let mut robot = Robot::new(4, 4, RobotType::Collector, WIDTH, HEIGHT, SEED);
        let mut grid = create_grid();
        let prev_len = robot.previous_positions.len();
        robot.move_towards(6, 4, &mut grid, WIDTH, HEIGHT);
        assert_eq!(robot.previous_positions.len(), prev_len + 1);
    }

    #[test]
    fn test_move_towards_within_bounds() {
        let mut robot = Robot::new(0, 0, RobotType::Collector, WIDTH, HEIGHT, SEED);
        let mut grid = create_grid();
        robot.move_towards(WIDTH + 5, HEIGHT + 5, &mut grid, WIDTH, HEIGHT);
        assert!(robot.x < WIDTH);
        assert!(robot.y < HEIGHT);
    }

    #[test]
    fn test_move_robot_explorator_changes_position() {
        let mut robot = Robot::new(1, 1, RobotType::Explorator, WIDTH, HEIGHT, SEED);
        let mut grid = create_grid();
        let mut base = Base::new(0, 0);
        let old_pos = (robot.x, robot.y);
        robot.move_robot(&mut grid, WIDTH, HEIGHT, &mut base);
        assert_ne!((robot.x, robot.y), old_pos);
    }

    #[test]
    fn test_move_robot_collector_changes_position() {
        let mut robot = Robot::new(1, 1, RobotType::Collector, WIDTH, HEIGHT, SEED);
        let mut grid = create_grid();
        let mut base = Base::new(0, 0);
        let old_pos = (robot.x, robot.y);
        robot.move_robot(&mut grid, WIDTH, HEIGHT, &mut base);
        assert_ne!((robot.x, robot.y), old_pos);
    }

    #[test]
    fn test_move_robot_scientist_changes_position() {
        let mut robot = Robot::new(1, 1, RobotType::Scientist, WIDTH, HEIGHT, SEED);
        let mut grid = create_grid();
        let mut base = Base::new(0, 0);
        let old_pos = (robot.x, robot.y);
        robot.move_robot(&mut grid, WIDTH, HEIGHT, &mut base);
        assert_ne!((robot.x, robot.y), old_pos);
    }

    #[test]
    fn test_rng_consistency() {
        let mut robot1 = Robot::new(0, 0, RobotType::Collector, WIDTH, HEIGHT, SEED);
        let mut robot2 = Robot::new(0, 0, RobotType::Collector, WIDTH, HEIGHT, SEED);
        let n1 = robot1.rng.gen::<u32>();
        let n2 = robot2.rng.gen::<u32>();
        assert_eq!(n1, n2);
    }

    #[test]
    fn test_inventory_not_exceed_capacity() {
        let mut robot = Robot::new(0, 0, RobotType::Collector, WIDTH, HEIGHT, SEED);
        for _ in 0..10 {
            if robot.inventory.len() < robot.max_capacity {
                robot.inventory.push(TileType::Mineral);
            }
        }
        assert!(robot.inventory.len() <= robot.max_capacity);
    }
}
