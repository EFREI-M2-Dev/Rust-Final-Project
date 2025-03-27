use super::robot::Robot;
use crate::map::{base::Base, TileType};
use crate::utils::debug_to_terminal::debug_to_terminal;

use rand::Rng;

pub struct Explorator;

impl Explorator {
    pub fn move_robot(
        robot: &mut Robot,
        grid: &mut Vec<Vec<TileType>>,
        width: usize,
        height: usize,
        base: &mut Base,
    ) {
        if robot.returning_to_base {
            robot.move_towards(robot.base.0, robot.base.1, grid, width, height);
            if robot.x == robot.base.0 && robot.y == robot.base.1 {
                debug_to_terminal("📡 Transmission des données à la base !");
                base.receive_resources(
                    robot.discovered_minerals.clone(),
                    robot.discovered_energy.clone(),
                    robot.discovered_plan.clone(),
                );
                robot.returning_to_base = false;
            }
            return;
        }

        let mut best_x = robot.x;
        let mut best_y = robot.y;
        let mut found_new_tile = false;

        let radius = 3;
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                let nx = robot.x as isize + dx;
                let ny = robot.y as isize + dy;

                if nx >= 0 && ny >= 0 && nx < width as isize && ny < height as isize {
                    let nx = nx as usize;
                    let ny = ny as usize;

                    if grid[ny][nx] == TileType::Mineral
                        && !robot.discovered_minerals.contains(&(nx, ny))
                    {
                        robot.discovered_minerals.push((nx, ny));
                        debug_to_terminal(&format!("💎 Minéral découvert à ({}, {})", nx, ny));
                        robot.returning_to_base = true;
                        return;
                    }

                    if grid[ny][nx] == TileType::Energy
                        && !robot.discovered_energy.contains(&(nx, ny))
                    {
                        robot.discovered_energy.push((nx, ny));
                        debug_to_terminal(&format!(
                            "⚡ Source d’énergie trouvée à ({}, {})",
                            nx, ny
                        ));
                        robot.returning_to_base = true;
                        return;
                    }

                    if grid[ny][nx] == TileType::Interest {
                        robot.discovered_plan.push((nx, ny));
                        debug_to_terminal(&format!(
                            "🧪 Point d'intérêt découvert à ({}, {})",
                            nx, ny
                        ));
                        robot.returning_to_base = true;
                        return;
                    }
                }
            }
        }

        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        for (dx, dy) in directions.iter() {
            let nx = robot.x as isize + dx;
            let ny = robot.y as isize + dy;

            if nx >= 0 && ny >= 0 && nx < width as isize && ny < height as isize {
                let nx = nx as usize;
                let ny = ny as usize;

                if !robot.visited_map[ny][nx] && grid[ny][nx] == TileType::Empty {
                    best_x = nx;
                    best_y = ny;
                    found_new_tile = true;
                    break;
                }
            }
        }

        if !found_new_tile {
            for _ in 0..10 {
                let (dx, dy) = directions[robot.rng.gen_range(0..4)];
                let nx = robot.x as isize + dx;
                let ny = robot.y as isize + dy;

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

        robot.x = best_x;
        robot.y = best_y;
        robot.visited_map[robot.y][robot.x] = true;
    }
}
