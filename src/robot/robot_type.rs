use ratatui::style::Color;

#[derive(Debug)]
pub enum RobotType {
    Explorator,
    Collector,
}

impl RobotType {
    pub fn to_char(&self) -> char {
        match self {
            RobotType::Explorator => 'R',
            RobotType::Collector => 'C',
        }
    }

    pub fn to_color(&self) -> Color {
        match self {
            RobotType::Explorator => Color::Red,
            RobotType::Collector => Color::Blue,
        }
    }
}
