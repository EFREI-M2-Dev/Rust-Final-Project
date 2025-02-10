use noise::{NoiseFn, Perlin};
use super::{Map, TileType};

const SCALE: f64 = 0.1;
const THRESHOLD: f64 = 0.3;

pub fn generate_map(width: usize, height: usize, seed: u32) -> Map {
    let perlin = Perlin::new(seed);
    let mut map = Map::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let noise_value = perlin.get([x as f64 * SCALE, y as f64 * SCALE]);
            map.grid[y][x] = if noise_value > THRESHOLD { TileType::Wall } else { TileType::Empty };
        }
    }

    map
}