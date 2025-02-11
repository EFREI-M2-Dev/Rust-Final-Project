mod screens;
mod map;

use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use std::io;

mod robot;
mod module;

use robot::{CollectorRobot, ExploratorRobot};
use crate::robot::traits::Robot;


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
    Map(screens::map::GameMap),
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
            AppState::Map(map) => map.should_exit(),
        }
    }

    fn draw(&self, frame: &mut Frame) {
        match &self.state {
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
                                    /* let mut map = screens::map::Map::new();
                                    let mut explorator = ExploratorRobot::new("Explorateur-01", 120.0);
                                    for _ in 0..10 {
                                        let pos = (explorator.position().0 as usize, explorator.position().1 as usize);
                                        map.base_map.display(pos);
                                        explorator.move_robot(1, 0, &mut map);
                                    } */
                                    self.state = AppState::Map(map.base_map);
                                }
                                _ => {}
                            }
                        }
                    }
                    AppState::Map(map) => map.handle_key_event(key_event),
                }
            }
            _ => {}
        };
        Ok(())
    }
}