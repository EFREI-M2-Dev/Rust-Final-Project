use ratatui::style::Color;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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

    pub fn cost(&self) -> (usize, usize, usize) {
        match self {
            RobotType::Explorator => (5, 3, 0),
            RobotType::Collector => (3, 5, 0),
            RobotType::Scientist => (2, 2, 5),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RobotModule {
    Sensor,
    Drill,
    Camera,
}
