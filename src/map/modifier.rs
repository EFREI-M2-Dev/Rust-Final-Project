use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
use super::{BaseMap, TileType, MapModifier};

pub fn add_random_elements(tile: TileType, density: f64, seed: u32) -> MapModifier {
    Box::new(move |map: &mut BaseMap| {
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

pub fn add_base_center() -> MapModifier {
    Box::new(move |map: &mut BaseMap| {
        let cx = map.width as isize / 2;
        let cy = map.height as isize / 2;
        let size = 3;

        let can_place_base = |x: isize, y: isize, map: &BaseMap| -> bool {
            if x < 0 || y < 0 || (x + size as isize) >= map.width as isize || (y + size as isize) >= map.height as isize {
                return false;
            }
            for dy in 0..size {
                for dx in 0..size {
                    if map.grid[(y + dy as isize) as usize][(x + dx as isize) as usize] != TileType::Empty {
                        return false;
                    }
                }
            }
            true
        };

        let mut best_x = cx;
        let mut best_y = cy;
        let mut found = false;

        'search: for radius in 0..map.width.max(map.height) as isize {
            for dy in -radius..=radius {
                for dx in -radius..=radius {
                    let nx = cx + dx;
                    let ny = cy + dy;

                    if can_place_base(nx, ny, map) {
                        best_x = nx;
                        best_y = ny;
                        found = true;
                        break 'search;
                    }
                }
            }
        }

        if found {
            for dy in 0..size {
                for dx in 0..size {
                    map.grid[(best_y + dy as isize) as usize][(best_x + dx as isize) as usize] = TileType::Base;
                }
            }
        }
    })
}