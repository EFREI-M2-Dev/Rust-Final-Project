use super::base::Base;
use super::tile_type::TileType;
use crate::robot::{robot_type::RobotModule, Robot, RobotType};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<TileType>>,
    pub robots: Vec<Robot>,
    pub fog: Vec<Vec<bool>>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![TileType::Empty; width]; height];
        let fog = vec![vec![false; width]; height];

        Map {
            width,
            height,
            grid,
            robots: vec![],
            fog,
        }
    }

    pub fn add_robot(&mut self, x: usize, y: usize, seed: u32) {
        let robot_configs = vec![
            (RobotType::Explorator, seed, None),
            (RobotType::Explorator, seed + 1, None),
            (RobotType::Collector, seed, None),
            (RobotType::Scientist, seed, None),
        ];

        for (robot_type, seed, module) in robot_configs {
            let mut robot = Robot::new(x, y, robot_type, self.width, self.height, seed);

            if let Some(robot_module) = module {
                robot.add_module(robot_module);
            }

            self.robots.push(robot);
        }

        self.reveal_area(x, y);
    }

    fn reveal_area(&mut self, x: usize, y: usize) {
        let radius = 3;
        for dy in -(radius as isize)..=(radius as isize) {
            for dx in -(radius as isize)..=(radius as isize) {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx >= 0 && ny >= 0 && nx < self.width as isize && ny < self.height as isize {
                    self.fog[ny as usize][nx as usize] = true;
                }
            }
        }
    }

    pub fn update_robots(&mut self, base: &mut Base) {
        let width = self.width;
        let height = self.height;

        let grid = Arc::new(Mutex::new(&mut self.grid));
        let base = Arc::new(Mutex::new(base));
        let updates = Arc::new(Mutex::new(Vec::new()));

        self.robots.par_iter_mut().for_each(|robot| {
            let mut grid = grid.lock().unwrap();
            let mut base = base.lock().unwrap();
            let mut updates = updates.lock().unwrap();

            robot.move_robot(&mut grid, width, height, &mut base);

            if let RobotType::Explorator = robot.robot_type {
                updates.push((robot.x, robot.y));
            }
        });

        let updates = Arc::try_unwrap(updates).unwrap().into_inner().unwrap();
        for (x, y) in updates {
            self.reveal_area(x, y);
        }
    }

    pub fn count_robots_by_type(&self) -> std::collections::HashMap<RobotType, usize> {
        let mut robot_counts = std::collections::HashMap::new();

        for robot in &self.robots {
            let count = robot_counts.entry(robot.robot_type.clone()).or_insert(0);
            *count += 1;
        }

        robot_counts
    }
}

pub type MapModifier = Box<dyn FnMut(&mut Map)>;
