use rand::Rng;
use super::{Map, TileType};

pub fn add_minerals(map: &mut Map, density: f64) {
    let mut rng = rand::thread_rng();

    for y in 0..map.height {
        for x in 0..map.width {
            if map.grid[y][x] == TileType::Empty && rng.gen::<f64>() < density {
                map.grid[y][x] = TileType::Mineral;
            }
        }
    }
}