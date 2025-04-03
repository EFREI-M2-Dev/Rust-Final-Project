use super::robot::Robot;
use super::robot_type::RobotModule;
use crate::map::{base::Base, TileType};
use crate::utils::debug_to_terminal::debug_to_terminal;

pub struct Collector;

impl Collector {
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
                debug_to_terminal(&format!(
                    "[Collector] \tDéposé {} ressources à la base !",
                    robot.inventory.len()
                ));

                let mineral_count = robot
                    .inventory
                    .iter()
                    .filter(|&&r| r == TileType::Mineral)
                    .count();
                let energy_count = robot
                    .inventory
                    .iter()
                    .filter(|&&r| r == TileType::Energy)
                    .count();
                let mut ressources_deposited: Vec<(usize, usize)> =
                    robot.inventory.iter().map(|_| (robot.x, robot.y)).collect();

                base.receive_inventory(mineral_count, energy_count, 0, &mut ressources_deposited);
                robot.inventory.clear();
                robot.returning_to_base = false;
                robot.target = None;
            }
            return;
        }

        if robot.target.is_none() {
            if let Some(mineral_pos) = base.get_mineral_target() {
                debug_to_terminal(&format!(
                    "[Collector] \tNouveau minerai assigné au robot : {:?}",
                    mineral_pos
                ));
                robot.target = Some(mineral_pos);
            } else if let Some(energy_pos) = base.get_energy_target() {
                debug_to_terminal(&format!(
                    "[Collector] \tNouvelle source d’énergie assignée au robot : {:?}",
                    energy_pos
                ));
                robot.target = Some(energy_pos);
            }
        }

        let (tx, ty) = robot.target.unwrap_or(robot.base);

        let adjacent_positions = [
            (robot.x.wrapping_sub(1), robot.y),
            (robot.x + 1, robot.y),
            (robot.x, robot.y.wrapping_sub(1)),
            (robot.x, robot.y + 1),
        ];

        for (nx, ny) in adjacent_positions.iter() {
            if *nx < width && *ny < height {
                if (*nx, *ny) == (tx, ty)
                    && (grid[*ny][*nx] == TileType::Mineral || grid[*ny][*nx] == TileType::Energy)
                {
                    debug_to_terminal(&format!(
                        "[Collector] \tRessource collectée à ({}, {})",
                        *nx, *ny
                    ));

                    robot.inventory.push(grid[*ny][*nx]);
                    grid[*ny][*nx] = TileType::Empty;

                    if robot.inventory.len() >= robot.max_capacity {
                        debug_to_terminal("[Collector] \tInventaire plein ! Retour à la base...");
                        robot.returning_to_base = true;
                    } else {
                        robot.target = None;
                    }
                    return;
                }
            }
        }

        if robot.modules.contains(&RobotModule::Drill) {
            for (nx, ny) in adjacent_positions.iter() {
                if *nx < width && *ny < height {
                    if grid[*ny][*nx] == TileType::Mountain {
                        debug_to_terminal(&format!(
                            "[Collector] \tForage à ({}, {}) pour découvrir des ressources enfouies",
                            *nx, *ny
                        ));
                        grid[*ny][*nx] = TileType::Empty;
                        robot.move_towards(*nx, *ny, grid, width, height);
                        return;
                    }
                }
            }
        }

        robot.move_towards(tx, ty, grid, width, height);
    }
}
