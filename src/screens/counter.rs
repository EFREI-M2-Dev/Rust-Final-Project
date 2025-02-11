use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{text::Line, widgets::{Block, Paragraph}, style::Stylize};

#[derive(Debug)]
pub struct Counter {
    pub exit: bool,
    counter: u8,
    location: &'static str,
}

impl Counter {
    pub fn new(location: &'static str) -> Self {
        Self {
            exit: false,
            counter: 0,
            location,
        }
    }

    pub fn draw(&self, frame: &mut ratatui::Frame) {
        let title = Line::from(format!(" Exploration de {} ", self.location).bold());
        let instructions = Line::from(" Appuyez sur P pour diminuer, L pour augmenter, Q pour quitter ".blue());

        let block = Block::bordered().title(title.centered()).title_bottom(instructions.centered());
        let counter_text = Line::from(format!("Valeur: {}", self.counter).yellow());

        let paragraph = Paragraph::new(counter_text).block(block).centered();
        frame.render_widget(paragraph, frame.area());
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Char('p') => self.counter = self.counter.saturating_sub(1),
            KeyCode::Char('l') => self.counter = self.counter.saturating_add(1),
            _ => {}
        }
    }

    pub fn should_exit(&self) -> bool {
        self.exit
    }
}
