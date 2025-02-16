use super::TileType;
use rand::Rng;

#[derive(Debug)]
pub struct Robot {
    pub x: usize,
    pub y: usize,
}

impl Robot {
    pub fn new(x: usize, y: usize) -> Self {
        Robot { x, y }
    }

    pub fn move_randomly(&mut self, grid: &Vec<Vec<TileType>>, width: usize, height: usize) {
        let mut rng = rand::thread_rng();
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        for _ in 0..10 {
            let (dx, dy) = directions[rng.gen_range(0..4)];
            let nx = self.x as isize + dx;
            let ny = self.y as isize + dy;

            if nx >= 0 && ny >= 0 && nx < width as isize && ny < height as isize {
                let nx = nx as usize;
                let ny = ny as usize;

                if grid[ny][nx] == TileType::Empty {
                    self.x = nx;
                    self.y = ny;
                    break;
                }
            }
        }
    }
}
