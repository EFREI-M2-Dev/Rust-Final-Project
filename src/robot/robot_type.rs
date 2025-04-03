use ratatui::style::Color;

#[derive(Debug)]
pub enum RobotType {
    Explorator,
    Collector,
    Scientist,
}

impl RobotType {
    pub fn to_char(&self) -> char {
        match self {
            RobotType::Explorator => 'R',
            RobotType::Collector => 'C',
            RobotType::Scientist => 'S',
        }
    }

    pub fn to_color(&self) -> Color {
        match self {
            RobotType::Explorator => Color::Red,
            RobotType::Collector => Color::Blue,
            RobotType::Scientist => Color::Green,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RobotModule {
    Sensor,
    Drill,
    Camera,
}
