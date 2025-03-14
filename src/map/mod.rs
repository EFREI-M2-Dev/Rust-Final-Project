use base::Base;
use ratatui::style::Color;

pub mod base;
pub mod generator;
pub mod modifier;
mod robot;

use robot::{Robot, RobotType};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileType {
    Empty,
    Mountain,
    Mineral,
    Water,
    Sand,
    Base,
    Energy,
    Interest,
}

impl TileType {
    pub fn to_char(&self) -> char {
        match self {
            TileType::Empty => ' ',
            TileType::Mountain => '^',
            TileType::Mineral => 'M',
            TileType::Water => '~',
            TileType::Sand => '.',
            TileType::Base => 'B',
            TileType::Energy => 'E',
            TileType::Interest => 'I',
        }
    }

    pub fn to_color(&self) -> Color {
        match self {
            TileType::Empty => Color::Reset,
            TileType::Mountain => Color::Rgb(156, 81, 23),
            TileType::Mineral => Color::Yellow,
            TileType::Water => Color::Blue,
            TileType::Sand => Color::Rgb(194, 178, 128),
            TileType::Base => Color::Rgb(250, 90, 218),
            TileType::Energy => Color::Rgb(250, 90, 218),
            TileType::Interest => Color::Rgb(250, 90, 218),
        }
    }
}

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
        self.robots.push(Robot::new(
            x,
            y,
            RobotType::Explorator,
            self.width,
            self.height,
            seed,
        ));
        self.robots.push(Robot::new(
            x,
            y,
            RobotType::Collector,
            self.width,
            self.height,
            seed,
        ));
        self.reveal_area(x, y);
    }

    /* pub fn update_fog(&mut self) {
        let robot_positions: Vec<(usize, usize)> = self.robots.iter().map(|r| (r.x, r.y)).collect();

        for (x, y) in robot_positions {
            self.reveal_area(x, y);
        }
    } */

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
        let previous_fog = self.fog.clone();

        let mut updates = Vec::new();

        for robot in &mut self.robots {
            robot.move_robot(&mut self.grid, width, height, base);

            if let RobotType::Explorator = robot.robot_type {
                updates.push((robot.x, robot.y));
            }
        }

        for (x, y) in updates {
            self.reveal_area(x, y);
        }

        // for y in 0..height {
        //     for x in 0..width {
        //         if previous_fog[y][x] == false && self.fog[y][x] == true {
        //             if self.grid[y][x] == TileType::Mineral {
        //                 for robot in &mut self.robots {
        //                     if robot.target.is_none() {
        //                         if let RobotType::Collector = robot.robot_type {
        //                             robot.target = Some((x, y));
        //                         }
        //                     }
        //                 }
        //             }
        //         }
        //     }
        // }
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.fog[y][x] {
                    if let Some(robot) = self.robots.iter().find(|r| r.x == x && r.y == y) {
                        match robot.robot_type {
                            RobotType::Explorator => print!("R"),
                            RobotType::Collector => print!("C"),
                        }
                    } else {
                        print!("{}", self.grid[y][x].to_char());
                    }
                } else {
                    print!("#");
                }
            }
            println!();
        }
    }
}

pub type MapModifier = Box<dyn FnMut(&mut Map)>;
