use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::Alignment;
use crate::map::generator::generate_map;
use crate::map::modifier::add_random_elements;
use crate::map::TileType;
use ratatui::text::{Line, Span};
use ratatui::style::{Color, Style};

#[derive(Debug)]
pub struct Map {
    pub exit: bool,
    pub grid: Vec<Vec<TileType>>,
}


impl Map {
    pub fn new() -> Self {
        let width = 100;
        let height = 50;
        let seed = 42;

        let modifiers = vec![
            add_random_elements(TileType::Mineral, 0.01, seed),
        ];

        let generated_map = generate_map(width, height, seed, modifiers);

        Self {
            exit: false,
            grid: generated_map.grid,
        }
    }

    pub fn draw(&self, frame: &mut ratatui::Frame) {
        let mut map_lines = Vec::new();
        for row in &self.grid {
            let mut line = Line::default();
            for tile in row {
                let (ch, color) = match tile {
                    TileType::Empty => (' ', Color::Reset),
                    TileType::Mountain => ('^', Color::Green),
                    TileType::Mineral => ('M', Color::Yellow),
                    TileType::Water => ('~', Color::Blue),
                    TileType::Sand => ('.', Color::Rgb(194, 178, 128)),
                };
                line.spans.push(Span::styled(ch.to_string(), Style::default().fg(color)));
            }
            map_lines.push(line);
        }

        let map_paragraph = ratatui::widgets::Paragraph::new(map_lines)
            .block(ratatui::widgets::Block::default().title("Map").title_alignment(Alignment::Center).borders(ratatui::widgets::Borders::ALL));

        frame.render_widget(map_paragraph, frame.size());
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => {
                self.exit = true;
            }
            _ => {}
        }
    }

    pub fn should_exit(&self) -> bool {
        self.exit
    }
}