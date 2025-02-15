mod map;
mod screens;

use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use std::io;

mod robot;
mod module;

use robot::{CollectorRobot, ExploratorRobot};
use crate::robot::traits::Robot; */

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::new().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug)]
pub struct App<'a> {
    state: AppState<'a>,
}

#[derive(Debug)]
enum AppState<'a> {
    Home(screens::home::Home),
    Map(screens::map::Map<'a>),
}

impl Default for App<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl App<'_> {
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
            AppState::Map(map) => map.should_exit(),
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        match &mut self.state {
            AppState::Home(home) => home.draw(frame),
            AppState::Map(map) => map.draw(frame),
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match &mut self.state {
                    AppState::Home(home) => {
                        if let Some(selection) = home.handle_key_event(key_event) {
                            match selection {
                                "Nouvelle partie" => {
                                    self.state = AppState::Map(screens::map::Map::new())
                                }
                                _ => {}
                            }
                        }
                    }
                    AppState::Map(map) => {
                        map.handle_key_event(key_event);
                        if map.return_back {
                            self.state = AppState::Home(screens::home::Home::new());
                        }
                    }

                }
            }
            _ => {}
        };
        Ok(())
    }
}
