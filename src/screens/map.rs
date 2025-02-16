use crate::map::generator::generate_map;
use crate::map::modifier::{add_base_center, add_random_elements};
use crate::map::{BaseMap, TileType};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::Alignment;
use ratatui::style::{Style, Stylize};
use ratatui::text::{Line, Span};

#[derive(Debug)]
pub struct Map<'a> {
    pub return_back: bool,
    pub base_map: BaseMap,
    pub map_lines: Vec<Line<'a>>,
    pub viewport_x: usize,
    pub viewport_y: usize,
    pub viewport_width: usize,
    pub viewport_height: usize,
}

impl<'a> Map<'a> {
    pub fn new() -> Self {
        let width = 300;
        let height = 75;
        let seed = 42;

        let modifiers = vec![
            add_base_center(),
            add_random_elements(TileType::Mineral, 0.01, seed),
        ];

        let generated_map = generate_map(width, height, seed, modifiers);

        let mut map_lines = Vec::new();
        for row in &generated_map.grid {
            let mut line = Line::default();
            for tile in row {
                let ch = tile.to_char();
                let color = tile.to_color();
                line.spans
                    .push(Span::styled(ch.to_string(), Style::default().fg(color)));
            }
            map_lines.push(line);
        }

        Self {
            return_back: false,
            base_map: generated_map,
            map_lines,
            viewport_x: 0,
            viewport_y: 0,
            viewport_width: 0,
            viewport_height: 0,
        }
    }

    pub fn draw(&mut self, frame: &mut ratatui::Frame) {
        let terminal_size = frame.area();

        if self.viewport_width == 0 || self.viewport_height == 0 {
            self.viewport_width = terminal_size.width as usize;
            self.viewport_height = terminal_size.height as usize;

            self.viewport_x = (self.base_map.width.saturating_sub(self.viewport_width)) / 2;
            self.viewport_y = (self.base_map.height.saturating_sub(self.viewport_height)) / 2;
        }

        let mut visible_lines = Vec::new();
        for y in self.viewport_y..(self.viewport_y + self.viewport_height) {
            if y < self.map_lines.len() {
                let mut visible_line = Line::default();
                let line = &self.map_lines[y];
                for x in self.viewport_x..(self.viewport_x + self.viewport_width) {
                    if x < line.spans.len() {
                        visible_line.spans.push(line.spans[x].clone());
                    }
                }
                visible_lines.push(visible_line);
            }
        }

        let instructions = Line::from(
            " Utiliser les flèches pour naviguer ◀ ▶, ▲ ▼ | ESC pour revenir au menu ".red(),
        );
        let exit = Line::from(" Esc pour quitter ".red());
        let map_paragraph = ratatui::widgets::Paragraph::new(visible_lines).block(
            ratatui::widgets::Block::default()
                .title(" Map ")
                .bold()
                .title_alignment(Alignment::Center)
                .title_bottom(instructions.centered())
                .title_style(Style::default().fg(ratatui::style::Color::Yellow))
                .borders(ratatui::widgets::Borders::ALL),
        );

        frame.render_widget(map_paragraph, terminal_size);
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => {
                self.return_back = true;
            }
            KeyCode::Up => {
                self.viewport_y = self.viewport_y.saturating_sub(5);
            }
            KeyCode::Down => {
                if self.viewport_y + self.viewport_height < self.map_lines.len() {
                    self.viewport_y = (self.viewport_y + 5)
                        .min(self.map_lines.len().saturating_sub(self.viewport_height));
                }
            }
            KeyCode::Left => {
                self.viewport_x = self.viewport_x.saturating_sub(5);
            }
            KeyCode::Right => {
                if self.viewport_x + self.viewport_width < self.map_lines[0].spans.len() {
                    self.viewport_x = (self.viewport_x + 5).min(
                        self.map_lines[0]
                            .spans
                            .len()
                            .saturating_sub(self.viewport_width),
                    );
                }
            }
            _ => {}
        }
    }

    pub fn should_exit(&self) -> bool {
        self.return_back
    }
}
