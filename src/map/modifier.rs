use super::base::Base;
use super::{Map, MapModifier, TileType};
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

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

pub fn add_base(base: &Base) -> MapModifier {
    let base_x = base.x;
    let base_y = base.y;
    let half_size = Base::SIZE as isize / 2;

    Box::new(move |map: &mut Map| {
        for dy in -half_size..=half_size {
            for dx in -half_size..=half_size {
                let bx = base_x as isize + dx;
                let by = base_y as isize + dy - 1;

                if bx >= 0 && by >= 0 && bx < map.width as isize && by < map.height as isize {
                    map.grid[by as usize][bx as usize] = TileType::Base;
                }
            }
        }
    })
}
