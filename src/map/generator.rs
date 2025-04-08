use super::{Map, MapModifier, TileType};
use noise::{NoiseFn, Perlin};

const SCALE: f64 = 0.05;
const THRESHOLDS: [(TileType, f64); 4] = [
    (TileType::Water, -0.2),
    (TileType::Sand, -0.1),
    (TileType::Empty, 0.2),
    (TileType::Mountain, 0.9),
];

pub fn generate_base_map(width: usize, height: usize, seed: u32) -> Map {
    let perlin = Perlin::new(seed);
    let mut map = Map::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let noise_value = perlin.get([x as f64 * SCALE, y as f64 * SCALE]);
            let noise_value = noise_value * 0.3;

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

pub fn generate_map(
    width: usize,
    height: usize,
    seed: u32,
    mut modifiers: Vec<MapModifier>,
) -> Map {
    let mut map = generate_base_map(width, height, seed);

    for modifier in &mut modifiers {
        modifier(&mut map);
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    const WIDTH: usize = 20;
    const HEIGHT: usize = 30;
    const SEED: u32 = 100;

    #[test]
    fn test_generate_map_dimensions() {
        let map = generate_map(WIDTH, HEIGHT, SEED, vec![]);
        assert_eq!(map.grid.len(), HEIGHT);
        assert_eq!(map.grid[0].len(), WIDTH);
    }

    #[test]
    fn test_generate_map_reproducibility() {
        let map1 = generate_map(WIDTH, HEIGHT, SEED, vec![]);
        let map2 = generate_map(WIDTH, HEIGHT, SEED, vec![]);
        assert_eq!(map1.grid, map2.grid);
    }
}
