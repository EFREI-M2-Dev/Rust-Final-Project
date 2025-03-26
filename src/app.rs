use crate::input;
use crate::screens;
use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use std::io;

pub struct App<'a> {
    state: AppState<'a>,
}

pub enum AppState<'a> {
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
        input::handle_events(&mut self.state)
    }
}
