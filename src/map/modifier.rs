use rand::Rng;
use super::{Map, TileType, MapModifier};

pub fn add_random_elements(tile: TileType, density: f64) -> MapModifier {
    Box::new(move |map: &mut Map| {
        let mut rng = rand::thread_rng();

        for y in 0..map.height {
            for x in 0..map.width {
                if map.grid[y][x] == TileType::Empty && rng.gen::<f64>() < density {
                    map.grid[y][x] = tile;
                }
            }
        }
    })
}