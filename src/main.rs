#[macro_export]
macro_rules! debug_println {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            println!($($arg)*);
        }
    };
}

mod map;
mod utils;

use crate::map::base::Base;
use crate::map::generator::generate_map;
use crate::map::modifier::{add_base, add_random_elements};
use crate::map::robot::RobotType;
use crate::map::TileType;

use crossterm::{
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use std::io::{self, stdout};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let width = 100;
    let height = 30;
    let seed = 57;

    let mut map = generate_map(width, height, seed, vec![]);

    let base_position =
        Base::find_free_position(&map.grid).expect("Aucune place libre pour la base !");
    let mut base = Base::new(base_position.0, base_position.1);

    let modifiers = vec![
        add_base(&base),
        add_random_elements(TileType::Mineral, 0.01, seed),
        add_random_elements(TileType::Energy, 0.006, seed),
        add_random_elements(TileType::Interest, 0.003, seed),
    ];

    map = generate_map(width, height, seed, modifiers);
    map.add_robot(base.x, base.y, seed);

    loop {
        map.update_robots(&mut base);

        terminal.draw(|f| draw_map(f, &map)).unwrap();

        if event::poll(std::time::Duration::from_millis(100))? {
            if let event::Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
                ..
            }) = event::read()?
            {
                break;
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn draw_map(frame: &mut Frame, map: &map::Map) {
    let map_str: Vec<String> = map
        .grid
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, tile)| {
                    if let Some(robot) = map.robots.iter().find(|r| r.x == x && r.y == y) {
                        match robot.robot_type {
                            RobotType::Explorator => "R".to_string(),
                            RobotType::Collector => "C".to_string(),
                        }
                    } else if map.fog[y][x] {
                        tile.to_char().to_string()
                    } else {
                        "#".to_string()
                    }
                })
                .collect()
        })
        .collect();

    let text = Paragraph::new(map_str.join("\n"))
        .block(Block::default().title(" Carte ").borders(Borders::ALL));

    let area = frame.size();
    frame.render_widget(text, area);
}
