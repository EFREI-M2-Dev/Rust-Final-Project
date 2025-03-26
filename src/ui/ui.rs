use crate::map::Map;
use crate::robot::RobotType;
use crossterm::{
    event::{self, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use std::io::{self, stdout, Stdout};

pub fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

pub fn teardown_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

pub fn draw_map(frame: &mut Frame, map: &Map) {
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

pub fn handle_input() -> bool {
    if event::poll(std::time::Duration::from_millis(100)).unwrap() {
        if let event::Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::NONE,
            ..
        }) = event::read().unwrap()
        {
            return true;
        }
    }
    false
}
