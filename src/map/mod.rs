use ratatui::style::Color;

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

    pub fn add_robot(&mut self, x: usize, y: usize) {
        self.robots.push(Robot::new(x, y, RobotType::Explorator));
        self.robots.push(Robot::new(x, y, RobotType::Collector));
        self.reveal_area(x, y);
    }

    pub fn update_fog(&mut self) {
        let robot_positions: Vec<(usize, usize)> = self.robots.iter().map(|r| (r.x, r.y)).collect();

        for (x, y) in robot_positions {
            self.reveal_area(x, y);
        }
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

    pub fn update_robots(&mut self) {
        let width = self.width;
        let height = self.height;
        let grid = self.grid.clone();
        let previous_fog = self.fog.clone();

        let mut updates = Vec::new();

        for robot in &mut self.robots {
            let previous_x = robot.x;
            let previous_y = robot.y;

            robot.move_robot(&grid, width, height);

            if let RobotType::Explorator = robot.robot_type {
                updates.push((robot.x, robot.y));
            }
        }

        for (x, y) in updates {
            self.reveal_area(x, y);
        }

        for y in 0..height {
            for x in 0..width {
                if previous_fog[y][x] == false && self.fog[y][x] == true {
                    if self.grid[y][x] == TileType::Mineral {
                        println!("Minerais découvert en position {}, {}", x, y);
                    }
                }
            }
        }
    }

    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.fog[y][x] {
                    if self.robots.iter().any(|r| r.x == x && r.y == y) {
                        print!("R");
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
