use crate::map::TileType;

#[derive(Debug)]
pub struct Base {
    pub x: usize,
    pub y: usize,
    pub discovered_minerals: Vec<(usize, usize)>,
    pub discovered_energy: Vec<(usize, usize)>,
    pub stored_minerals: usize,
    pub stored_energy: usize,
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

    pub fn receive_inventory(&mut self, mineral_count: usize, energy_count: usize) {
        self.stored_minerals += mineral_count;
        self.stored_energy += energy_count;

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
        self.discovered_minerals.pop()
    }

    pub fn get_energy_target(&mut self) -> Option<(usize, usize)> {
        self.discovered_energy.pop()
    }

    pub fn new(x: usize, y: usize) -> Self {
        Base {
            x,
            y,
            discovered_minerals: Vec::new(),
            discovered_energy: Vec::new(),
            stored_minerals: 0,
            stored_energy: 0,
        }
    }
}
