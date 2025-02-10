pub mod generator;
pub mod modifier;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileType {
    Empty,
    Wall, 
    Mineral,
}

impl TileType {
    pub fn to_char(&self) -> char {
        match self {
            TileType::Empty => '.',
            TileType::Wall => '#',
            TileType::Mineral => 'M',
        }
    }
}

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<TileType>>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![vec![TileType::Empty; width]; height],
        }
    }

    pub fn print(&self) {
        for row in &self.grid {
            for tile in row {
                print!("{}", tile.to_char());
            }
            println!();
        }
    }
}

pub type MapModifier = Box<dyn FnMut(&mut Map)>;