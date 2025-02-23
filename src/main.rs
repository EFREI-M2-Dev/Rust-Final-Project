mod screens;

use crossterm::event::{self, Event, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use std::io;

/* mod robot;
mod module;

use robot::{CollectorRobot, ExploratorRobot};
use crate::robot::traits::Robot; */

// fn main() -> io::Result<()> {
//     let mut terminal = ratatui::init();
//     let app_result = App::new().run(&mut terminal);
//     ratatui::restore();
//     app_result
// }

mod map;

use crate::map::generator::generate_map;
use crate::map::modifier::{add_base, add_random_elements};
use map::base::Base;
use map::TileType;

fn main() {
    let width = 30;
    let height = 30;
    let seed = 10;

    let mut map = generate_map(width, height, seed, vec![]);

    let base_position =
        Base::find_free_position(&map.grid).expect("Aucune place libre pour la base !");
    let base = Base::new(base_position.0, base_position.1);

    let modifiers = vec![
        add_base(&base),
        add_random_elements(TileType::Mineral, 0.01, seed),
    ];

    map = generate_map(width, height, seed, modifiers);

    map.add_robot(base.x, base.y);

    println!("Base position: {:?}", base_position);

    loop {
        map.update_robots();
        map.print();
        println!("=====================");
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
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
