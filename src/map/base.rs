use crate::map::TileType;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Base {
    pub x: usize,
    pub y: usize,
    pub discovered_minerals: Vec<(usize, usize)>,
    pub discovered_energy: Vec<(usize, usize)>,
    pub stored_minerals: usize,
    pub stored_energy: usize,
    collected_minerals: HashSet<(usize, usize)>,
    collected_energies: HashSet<(usize, usize)>,
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

        println!(
            "📡 Base a reçu {} minerais et {} sources d’énergie !",
            self.discovered_minerals.len(),
            self.discovered_energy.len()
        );
    }

    pub fn receive_inventory(
        &mut self,
        mineral_count: usize,
        energy_count: usize,
        ressources_deposited: &mut Vec<(usize, usize)>,
    ) {
        self.stored_minerals += mineral_count;
        self.stored_energy += energy_count;

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

        println!(
            "🧳 Base a reçu {} minerais et {} sources d’énergie !",
            mineral_count, energy_count
        );

        println!(
            "📦 Inventaire total → Minerais: {}, Énergie: {}",
            self.stored_minerals, self.stored_energy
        );
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

    pub fn new(x: usize, y: usize) -> Self {
        Base {
            x,
            y,
            discovered_minerals: Vec::new(),
            discovered_energy: Vec::new(),
            stored_minerals: 0,
            stored_energy: 0,
            collected_minerals: HashSet::new(),
            collected_energies: HashSet::new(),
        }
    }
}
