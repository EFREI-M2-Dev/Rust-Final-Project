use super::{base::Base, TileType};

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
    pub visited_map: Vec<Vec<bool>>,
    pub discovered_minerals: Vec<(usize, usize)>,
    pub discovered_energy: Vec<(usize, usize)>,
    pub returning_to_base: bool,
    pub inventory: Vec<TileType>,
    pub max_capacity: usize,
}

impl Robot {
    pub fn new(x: usize, y: usize, robot_type: RobotType, width: usize, height: usize) -> Self {
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
        }
    }

    pub fn move_robot(
        &mut self,
        grid: &mut Vec<Vec<TileType>>,
        width: usize,
        height: usize,
        base: &mut Base,
    ) {
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        match self.robot_type {
            RobotType::Explorator => {
                if self.returning_to_base {
                    self.move_towards(self.base.0, self.base.1, grid, width, height);
                    if self.x == self.base.0 && self.y == self.base.1 {
                        println!("📡 Transmission des données à la base !");
                        base.receive_resources(
                            self.discovered_minerals.clone(),
                            self.discovered_energy.clone(),
                        );
                        self.returning_to_base = false;
                    }
                    return;
                }

                let mut best_x = self.x;
                let mut best_y = self.y;
                let mut found_new_tile = false;

                for (dx, dy) in directions.iter() {
                    let nx = self.x as isize + dx;
                    let ny = self.y as isize + dy;

                    if nx >= 0 && ny >= 0 && nx < width as isize && ny < height as isize {
                        let nx = nx as usize;
                        let ny = ny as usize;

                        if grid[ny][nx] == TileType::Mineral {
                            if !self.discovered_minerals.contains(&(nx, ny)) {
                                self.discovered_minerals.push((nx, ny));
                                println!("💎 Minéral découvert à ({}, {})", nx, ny);
                                self.returning_to_base = true;
                                return;
                            }
                        }

                        if grid[ny][nx] == TileType::Energy {
                            if !self.discovered_energy.contains(&(nx, ny)) {
                                self.discovered_energy.push((nx, ny));
                                println!("⚡ Source d’énergie trouvée à ({}, {})", nx, ny);
                                self.returning_to_base = true;
                                return;
                            }
                        }

                        if !self.visited_map[ny][nx] && grid[ny][nx] == TileType::Empty {
                            best_x = nx;
                            best_y = ny;
                            found_new_tile = true;
                            break;
                        }
                    }
                }

                if !found_new_tile {
                    use rand::Rng;
                    let mut rng = rand::thread_rng();

                    for _ in 0..10 {
                        let (dx, dy) = directions[rng.gen_range(0..4)];
                        let nx = self.x as isize + dx;
                        let ny = self.y as isize + dy;

                        if nx >= 0 && ny >= 0 && nx < width as isize && ny < height as isize {
                            let nx = nx as usize;
                            let ny = ny as usize;

                            if grid[ny][nx] == TileType::Empty {
                                best_x = nx;
                                best_y = ny;
                                break;
                            }
                        }
                    }
                }

                self.x = best_x;
                self.y = best_y;
                self.visited_map[self.y][self.x] = true;
            }

            RobotType::Collector => {
                if self.returning_to_base {
                    self.move_towards(self.base.0, self.base.1, grid, width, height);
                    if self.x == self.base.0 && self.y == self.base.1 {
                        println!(
                            "🏠 Robot Collector a déposé {} ressources à la base !",
                            self.inventory.len()
                        );

                        let mineral_count = self
                            .inventory
                            .iter()
                            .filter(|&&r| r == TileType::Mineral)
                            .count();
                        let energy_count = self
                            .inventory
                            .iter()
                            .filter(|&&r| r == TileType::Energy)
                            .count();
                        let mut ressources_deposited: Vec<(usize, usize)> =
                            self.inventory.iter().map(|_| (self.x, self.y)).collect();

                        base.receive_inventory(
                            mineral_count,
                            energy_count,
                            &mut ressources_deposited,
                        );
                        self.inventory.clear();
                        self.returning_to_base = false;
                        self.target = None;
                    }
                    return;
                }

                if self.target.is_none() {
                    if let Some(mineral_pos) = base.get_mineral_target() {
                        println!("🎯 Nouveau minerai assigné au robot : {:?}", mineral_pos);
                        self.target = Some(mineral_pos);
                    } else if let Some(energy_pos) = base.get_energy_target() {
                        println!(
                            "⚡ Nouvelle source d’énergie assignée au robot : {:?}",
                            energy_pos
                        );
                        self.target = Some(energy_pos);
                    }
                }

                let (tx, ty) = self.target.unwrap_or(self.base);

                let adjacent_positions = [
                    (self.x.wrapping_sub(1), self.y),
                    (self.x + 1, self.y),
                    (self.x, self.y.wrapping_sub(1)),
                    (self.x, self.y + 1),
                ];

                for (nx, ny) in adjacent_positions.iter() {
                    if *nx < width && *ny < height {
                        if (*nx, *ny) == (tx, ty)
                            && (grid[*ny][*nx] == TileType::Mineral
                                || grid[*ny][*nx] == TileType::Energy)
                        {
                            println!("🛠️ Ressource collectée à ({}, {})", *nx, *ny);

                            self.inventory.push(grid[*ny][*nx]);
                            grid[*ny][*nx] = TileType::Empty;

                            if self.inventory.len() >= self.max_capacity {
                                println!("📦 Inventaire plein ! Retour à la base...");
                                self.returning_to_base = true;
                            } else {
                                self.target = None;
                            }
                            return;
                        }
                    }
                }

                self.move_towards(tx, ty, grid, width, height);
            }
        }
    }

    fn move_towards(
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
                    min_distance = distance;
                    best_x = nx;
                    best_y = ny;
                }
            }
        }

        self.x = best_x;
        self.y = best_y;
    }
}
