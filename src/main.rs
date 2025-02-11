mod screens;

use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use std::io;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug)]
pub struct App {
    state: AppState,
}

#[derive(Debug)]
enum AppState {
    Home(screens::home::Home),
    Counter(screens::counter::Counter),
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            state: AppState::Home(screens::home::Home::new()),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.should_exit() {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn should_exit(&self) -> bool {
        match &self.state {
            AppState::Home(home) => home.should_exit(),
            AppState::Counter(counter) => counter.should_exit(),
        }
    }

    fn draw(&self, frame: &mut Frame) {
        match &self.state {
            AppState::Home(home) => home.draw(frame),
            AppState::Counter(counter) => counter.draw(frame),
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match &mut self.state {
                    AppState::Home(home) => {
                        if let Some(selection) = home.handle_key_event(key_event) {
                            self.state = AppState::Counter(screens::counter::Counter::new(selection));
                        }
                    }
                    AppState::Counter(counter) => counter.handle_key_event(key_event),
                }
            }
            _ => {}
        };
        Ok(())
    }
}