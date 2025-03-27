use crate::map::Map;
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
    let styled_map_str: Vec<Line<'_>> = map
        .grid
        .iter()
        .enumerate()
        .map(|(y, row)| {
            let mut line = Line::default();
            row.iter().enumerate().for_each(|(x, tile)| {
                let (ch, color) =
                    if let Some(robot) = map.robots.iter().find(|r| r.x == x && r.y == y) {
                        (robot.robot_type.to_char(), robot.robot_type.to_color())
                    } else if map.fog[y][x] {
                        (tile.to_char(), tile.to_color())
                    } else {
                        ('#', Color::Reset)
                    };

                line.spans
                    .push(Span::styled(ch.to_string(), Style::default().fg(color)));
            });
            line
        })
        .collect();

    let instructions = Line::from(" Utiliser 'q' pour quitter ".red());

    let text = Paragraph::new(styled_map_str).block(
        Block::default()
            .title(" Carte ")
            .title_bottom(instructions.centered())
            .borders(Borders::ALL),
    );

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
