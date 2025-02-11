use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::Alignment;
use crate::map::generator::generate_map;
use crate::map::modifier::add_random_elements;
use ratatui::text::{Line, Span};
use ratatui::style::{Style};
use crate::map::{Map as BaseMap, TileType};


#[derive(Debug)]
pub struct Map {
    pub exit: bool,
    pub base_map: BaseMap,
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
            base_map: generated_map,
        }
    }

    pub fn draw(&self, frame: &mut ratatui::Frame) {
        let mut map_lines = Vec::new();
        for row in &self.base_map.grid {
            let mut line = Line::default();
            for tile in row {
                let ch = tile.to_char();
                let color = tile.to_color();
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