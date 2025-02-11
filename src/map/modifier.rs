use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
use super::{Map, TileType, MapModifier};

pub fn add_random_elements(tile: TileType, density: f64, seed: u32) -> MapModifier {
    Box::new(move |map: &mut Map| {
        let mut rng = StdRng::seed_from_u64(seed.into());

        for y in 0..map.height {
            for x in 0..map.width {
                if map.grid[y][x] == TileType::Empty && rng.gen::<f64>() < density {
                    map.grid[y][x] = tile;
                }
            }
        }
    })
}