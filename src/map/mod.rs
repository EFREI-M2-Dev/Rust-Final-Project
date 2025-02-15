use ratatui::style::Color;
use std::collections::HashSet;

pub mod generator;
pub mod modifier;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileType {
    Empty,
    Mountain,
    Mineral,
    Water,
    Sand,
    Base,
}

impl TileType {
    pub fn to_char(&self) -> char {
        match self {
            TileType::Empty => ' ',
            TileType::Mountain => '^',
            TileType::Mineral => 'M',
            TileType::Water => '~',
            TileType::Sand => '.',
            TileType::Base => 'B',
        }
    }

    pub fn to_color(&self) -> Color {
        match self {
            TileType::Empty => Color::Reset,
            TileType::Mountain => Color::Rgb(156, 81, 23),
            TileType::Mineral => Color::Yellow,
            TileType::Water => Color::Blue,
            TileType::Sand => Color::Rgb(194, 178, 128),
            TileType::Base => Color::Rgb(250, 90, 218),
        }
    }
}

#[derive(Debug)]
pub struct BaseMap {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<TileType>>,
    pub explored: HashSet<(usize, usize)>,
}

impl BaseMap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![vec![TileType::Empty; width]; height],
            explored: HashSet::new(),
        }
    }

    pub fn reveal(&mut self, x: usize, y: usize) {
        self.explored.insert((x, y));
    }

    pub fn reveal_area(&mut self, x: usize, y: usize, radius: usize) {
        let radius = radius as isize;  
        
        for dy in -radius..=radius { 
            for dx in -radius..=radius {
                let nx = (x as isize + dx).clamp(0, (self.width - 1) as isize) as usize;
                let ny = (y as isize + dy).clamp(0, (self.height - 1) as isize) as usize;

                self.reveal(nx, ny); 
            }
        }
    }

    pub fn display(&self, robot_pos: (usize, usize)) {
        for y in 0..self.height {
            for x in 0..self.width {
                if (x, y) == robot_pos {
                    print!("🤖");
                } else if self.explored.contains(&(x, y)) {
                    print!("{}", match self.grid[y][x] {
                        TileType::Water => '~',
                        TileType::Sand => '.',
                        TileType::Empty => ' ',
                        TileType::Mountain => '^',
                        TileType::Mineral => 'M',
                        TileType::Base => 'B',  
                    });
                } else {
                    print!("#");
                }
            }
            println!();
        }
        println!("====================");
    }
}

pub type MapModifier = Box<dyn FnMut(&mut Map)>;
