use crate::{
    map::TileType,
    robot::{Robot, RobotType},
    utils::debug_to_terminal::debug_to_terminal,
};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Base {
    pub x: usize,
    pub y: usize,
    pub discovered_minerals: Vec<(usize, usize)>,
    pub discovered_energy: Vec<(usize, usize)>,
    pub discovered_plans: Vec<(usize, usize)>,
    pub stored_minerals: usize,
    pub stored_energy: usize,
    pub stored_plans: usize,
    collected_minerals: HashSet<(usize, usize)>,
    collected_energies: HashSet<(usize, usize)>,
    collected_plans: HashSet<(usize, usize)>,
}

impl Base {
    pub const SIZE: usize = 1;

    fn can_place_base(grid: &Vec<Vec<TileType>>, x: isize, y: isize) -> bool {
        if x < 0
            || y < 0
            || (x + Self::SIZE as isize) >= grid[0].len() as isize
            || (y + Self::SIZE as isize) >= grid.len() as isize
        {
            return false;
        }
        for dy in 0..Self::SIZE {
            for dx in 0..Self::SIZE {
                if grid[(y + dy as isize) as usize][(x + dx as isize) as usize] != TileType::Empty {
                    return false;
                }
            }
        }
        true
    }

    pub fn find_free_position(grid: &Vec<Vec<TileType>>) -> Option<(usize, usize)> {
        let cx = grid[0].len() as isize / 2;
        let cy = grid.len() as isize / 2;

        for radius in 0..grid.len().max(grid[0].len()) as isize {
            for dy in -radius..=radius {
                for dx in -radius..=radius {
                    let nx = cx + dx;
                    let ny = cy + dy;

                    if Self::can_place_base(grid, nx, ny) {
                        return Some((nx as usize, ny as usize));
                    }
                }
            }
        }
        None
    }

    pub fn receive_resources(
        &mut self,
        minerals: Vec<(usize, usize)>,
        energy: Vec<(usize, usize)>,
        plans: Vec<(usize, usize)>,
    ) {
        for mineral in minerals {
            if !self.discovered_minerals.contains(&mineral) {
                self.discovered_minerals.push(mineral);
            }
        }

        for energy_source in energy {
            if !self.discovered_energy.contains(&energy_source) {
                self.discovered_energy.push(energy_source);
            }
        }

        for plan in &plans {
            if !self.discovered_plans.contains(&plan) {
                self.discovered_plans.push(*plan);
            }
        }

        // if plans.len() > 0 => send plans to earth in thread
        let plans_count = plans.len();

        if plans_count > 0 {
            let plans_clone = plans.clone();
            std::thread::spawn(move || {
                // Simulate sending plans to Earth
                std::thread::sleep(std::time::Duration::from_secs(2));
                debug_to_terminal(&format!(
                    "[Base] \tPlans scientifiques envoyés à la Terre ! ({})",
                    plans_clone.len()
                ));
            });
        }

        debug_to_terminal(&format!(
            "[Base] \tDécouvert au total: {} minerais, {} sources d’énergie et {} plans scientifiques !",
            self.discovered_minerals.len(),
            self.discovered_energy.len(),
            self.discovered_plans.len()
        ));
    }

    pub fn receive_inventory(
        &mut self,
        mineral_count: usize,
        energy_count: usize,
        plan_count: usize,
        ressources_deposited: &mut Vec<(usize, usize)>,
    ) {
        self.stored_minerals += mineral_count;
        self.stored_energy += energy_count;
        self.stored_plans += plan_count;

        for _ in 0..mineral_count {
            if let Some((x, y)) = self.discovered_minerals.pop() {
                ressources_deposited.push((x, y));
            }
        }

        for _ in 0..energy_count {
            if let Some((x, y)) = self.discovered_energy.pop() {
                ressources_deposited.push((x, y));
            }
        }

        for _ in 0..plan_count {
            if let Some((x, y)) = self.discovered_plans.pop() {
                ressources_deposited.push((x, y));
            }
        }

        debug_to_terminal(&format!(
            "[Base] \tA reçu {} minerais, {} sources d’énergie et {} découvertes scientifiques !",
            mineral_count, energy_count, plan_count
        ));

        debug_to_terminal(&format!(
            "[Base] \tInventaire total → Minerais: {}, Énergie: {}, Plans: {}",
            self.stored_minerals, self.stored_energy, self.stored_plans
        ));
    }

    pub fn get_mineral_target(&mut self) -> Option<(usize, usize)> {
        for (i, &mineral) in self.discovered_minerals.iter().enumerate() {
            if !self.collected_minerals.contains(&mineral) {
                self.collected_minerals.insert(mineral);
                return Some(self.discovered_minerals.remove(i));
            }
        }
        None
    }

    pub fn get_energy_target(&mut self) -> Option<(usize, usize)> {
        for (i, &energy) in self.discovered_energy.iter().enumerate() {
            if !self.collected_energies.contains(&energy) {
                self.collected_energies.insert(energy);
                return Some(self.discovered_energy.remove(i));
            }
        }
        None
    }

    pub fn get_plan_target(&mut self) -> Option<(usize, usize)> {
        for (i, &plan) in self.discovered_plans.iter().enumerate() {
            if !self.collected_plans.contains(&plan) {
                self.collected_plans.insert(plan);
                return Some(self.discovered_plans.remove(i));
            }
        }
        None
    }

    pub fn get_inventory(&mut self) -> (usize, usize, usize) {
        (self.stored_minerals, self.stored_energy, self.stored_plans)
    }

    pub fn try_create_robot(
        &mut self,
        robot_type: RobotType,
        map_width: usize,
        map_height: usize,
        seed: u32,
    ) -> Result<Robot, String> {
        let (min_req, energy_req, plan_req) = robot_type.cost();

        if self.stored_minerals < min_req
            || self.stored_energy < energy_req
            || self.stored_plans < plan_req
        {
            return Err("Ressources insuffisantes pour créer ce robot.".to_string());
        }

        self.stored_minerals -= min_req;
        self.stored_energy -= energy_req;
        self.stored_plans -= plan_req;

        Ok(Robot::new(
            self.x, self.y, robot_type, map_width, map_height, seed,
        ))
    }

    pub fn new(x: usize, y: usize) -> Self {
        Base {
            x,
            y,
            discovered_minerals: Vec::new(),
            discovered_energy: Vec::new(),
            discovered_plans: Vec::new(),
            stored_minerals: 0,
            stored_energy: 0,
            stored_plans: 0,
            collected_minerals: HashSet::new(),
            collected_energies: HashSet::new(),
            collected_plans: HashSet::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::tile_type::TileType;

    #[test]
    fn test_base_new() {
        let base = Base::new(5, 7);
        assert_eq!(base.x, 5);
        assert_eq!(base.y, 7);
    }

    #[test]
    fn test_find_free_position_returns_none_when_no_empty() {
        let grid = vec![
            vec![TileType::Mineral, TileType::Mineral],
            vec![TileType::Mineral, TileType::Mineral],
        ];
        let pos = Base::find_free_position(&grid);
        assert_eq!(pos, None);
    }
}
