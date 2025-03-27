use super::robot::Robot;
use crate::map::{base::Base, TileType};
use crate::utils::debug_to_terminal::debug_to_terminal;

pub struct Scientist;

impl Scientist {
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
                    "🏠 Robot Scientist à découvert le point d'intéret {} et à ramener le plan à la base !",
                    robot.inventory.len()
                ));

                let plan_count = robot
                    .inventory
                    .iter()
                    .filter(|&&r| r == TileType::Interest)
                    .count();
                let energy_count = robot
                    .inventory
                    .iter()
                    .filter(|&&r| r == TileType::Energy)
                    .count();
                let mut ressources_deposited: Vec<(usize, usize)> =
                    robot.inventory.iter().map(|_| (robot.x, robot.y)).collect();

                base.receive_inventory(0, energy_count, plan_count, &mut ressources_deposited);
                robot.inventory.clear();
                robot.returning_to_base = false;
                robot.target = None;
            }
            return;
        }

        if robot.target.is_none() {
            if let Some(interest_point_pos) = base.get_plan_target() {
                debug_to_terminal(&format!(
                    "🎯 Nouveau point d'intéret assigné au robot : {:?}",
                    interest_point_pos
                ));
                robot.target = Some(interest_point_pos);
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
                if (*nx, *ny) == (tx, ty) && (grid[*ny][*nx] == TileType::Interest) {
                    debug_to_terminal(&format!(
                        "🧪 Plan scientifique collectée à ({}, {})",
                        *nx, *ny
                    ));

                    robot.inventory.push(grid[*ny][*nx]);
                    grid[*ny][*nx] = TileType::Empty;

                    if robot.inventory.len() >= robot.max_capacity {
                        debug_to_terminal("📦 Inventaire plein ! Retour à la base...");
                        robot.returning_to_base = true;
                    } else {
                        robot.target = None;
                    }
                    return;
                }
            }
        }

        robot.move_towards(tx, ty, grid, width, height);
    }
}
