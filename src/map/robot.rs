use super::TileType;
use rand::Rng;

#[derive(Debug)]
pub enum RobotType {
    Explorator,
    Collector,
}
#[derive(Debug)]
pub struct Robot {
    pub x: usize,
    pub y: usize,
    pub robot_type: RobotType,
    pub base: (usize, usize),
    pub target: Option<(usize, usize)>,
}

impl Robot {
    pub fn new(x: usize, y: usize, robot_type: RobotType) -> Self {
        Robot {
            x,
            y,
            base: (x, y),
            robot_type,
            target: None,
        }
    }

    pub fn move_robot(&mut self, grid: &mut Vec<Vec<TileType>>, width: usize, height: usize) {
        let mut rng = rand::thread_rng();
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        match self.robot_type {
            RobotType::Explorator => {
                for _ in 0..10 {
                    let (dx, dy) = directions[rng.gen_range(0..4)];
                    let nx = self.x as isize + dx;
                    let ny = self.y as isize + dy;

                    if nx >= 0 && ny >= 0 && nx < width as isize && ny < height as isize {
                        let nx = nx as usize;
                        let ny = ny as usize;

                        if grid[ny][nx] == TileType::Empty {
                            self.x = nx;
                            self.y = ny;
                            break;
                        }
                    }
                }
            }
            RobotType::Collector => {
                let (tx, ty) = self.target.unwrap_or(self.base);

                let adjacent_positions = [
                    (self.x.wrapping_sub(1), self.y),
                    (self.x + 1, self.y),
                    (self.x, self.y.wrapping_sub(1)),
                    (self.x, self.y + 1),
                ];

                for (nx, ny) in adjacent_positions.iter() {
                    if *nx < width && *ny < height {
                        if grid[*ny][*nx] == TileType::Mineral {
                            // Ramasser le minerai
                            grid[*ny][*nx] = TileType::Empty;
                            println!("Minerai collecté à ({}, {})", *nx, *ny);

                            // Revenir à la base
                            self.target = Some(self.base);
                            break;
                        }
                    }
                }

                /* if self.x == tx && self.y == ty {
                    if grid[ty][tx] == TileType::Mineral {
                        // Supprimer le minerai
                        grid[ty][tx] = TileType::Empty;
                        println!("Minerai collecté à ({}, {})", tx, ty);

                        // Revenir à la base
                        self.target = Some(self.base);
                    }
                } */

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
                            min_distance = distance;
                            best_x = nx;
                            best_y = ny;
                        }
                    }
                }

                self.x = best_x;
                self.y = best_y;

                if self.x == self.base.0 && self.y == self.base.1 {
                    println!("Le robot est revenu à la base !");
                    self.target = None;
                }
            }
        }
    }

    /* pub fn move_randomly(&mut self, grid: &Vec<Vec<TileType>>, width: usize, height: usize) {
        let mut rng = rand::thread_rng();
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        for _ in 0..10 {
            let (dx, dy) = directions[rng.gen_range(0..4)];
            let nx = self.x as isize + dx;
            let ny = self.y as isize + dy;

            if nx >= 0 && ny >= 0 && nx < width as isize && ny < height as isize {
                let nx = nx as usize;
                let ny = ny as usize;

                if grid[ny][nx] == TileType::Empty {
                    self.x = nx;
                    self.y = ny;
                    break;
                }
            }
        }
    } */
}
