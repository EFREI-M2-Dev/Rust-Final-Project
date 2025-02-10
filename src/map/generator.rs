use noise::{NoiseFn, Perlin};

pub const WIDTH: usize = 50;
pub const HEIGHT: usize = 50;
const SCALE: f64 = 0.1;
const THRESHOLD: f64 = 0.3;

pub fn generate_map(seed: u32) -> [[u8; WIDTH]; HEIGHT] {
    let perlin = Perlin::new(seed);
    let mut map = [[0; WIDTH]; HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let noise_value = perlin.get([x as f64 * SCALE, y as f64 * SCALE]);
            map[y][x] = if noise_value > THRESHOLD { 1 } else { 0 };
        }
    }

    map
}

pub fn print_map(map: &[[u8; WIDTH]; HEIGHT]) {
    for row in map.iter() {
        for &cell in row.iter() {
            print!("{}", if cell == 1 { "#" } else { "." });
        }
        println!();
    }
}