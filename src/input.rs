use crate::app::AppState;
use crate::screens;
use crossterm::event::{self, Event, KeyEventKind};
use std::io;

pub fn handle_events(state: &mut AppState) -> io::Result<()> {
    match event::read()? {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => match state {
            AppState::Home(home) => {
                if let Some(selection) = home.handle_key_event(key_event) {
                    match selection {
                        "Nouvelle partie" => {
                            *state = AppState::Map(screens::map::Map::new());
                        }
                        _ => {}
                    }
                }
            }
            AppState::Map(map) => {
                map.handle_key_event(key_event);
                if map.return_back {
                    *state = AppState::Home(screens::home::Home::new());
                }
            }
        },
        _ => {}
    }
    Ok(())
}
