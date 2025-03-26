use ratatui::style::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileType {
    Empty,
    Mountain,
    Mineral,
    Water,
    Sand,
    Base,
    Energy,
    Interest,
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
            TileType::Energy => 'E',
            TileType::Interest => 'I',
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
            TileType::Energy => Color::Rgb(250, 90, 218),
            TileType::Interest => Color::Rgb(250, 90, 218),
        }
    }
}
