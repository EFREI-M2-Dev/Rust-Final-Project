use noise::{NoiseFn, Perlin};
use super::{Map, TileType, MapModifier};

const SCALE: f64 = 0.1;
const THRESHOLDS: [(TileType, f64); 4] = [
    (TileType::Water, -0.5),
    (TileType::Sand, -0.2),
    (TileType::Empty, 0.2),
    (TileType::Mountain, 1.0),
];

pub fn generate_base_map(width: usize, height: usize, seed: u32) -> Map {
    let perlin = Perlin::new(seed);
    let mut map = Map::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let noise_value = perlin.get([x as f64 * SCALE, y as f64 * SCALE]);

            let mut tile_type = TileType::Mountain;
            for (tile, threshold) in THRESHOLDS {
                if noise_value < threshold {
                    tile_type = tile;
                    break;
                }
            }

            map.grid[y][x] = tile_type;
        }
    }

    map
}

pub fn generate_map(width: usize, height: usize, seed: u32, mut modifiers: Vec<MapModifier>) -> Map {
    let mut map = generate_base_map(width, height, seed);

    for modifier in &mut modifiers {
        modifier(&mut map);
    }

    map
}