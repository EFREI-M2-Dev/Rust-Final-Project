use crate::map::base::Base;
use crate::map::Map;
use crate::ui::centered_rect::centered_rect;
use crate::utils::debug_to_terminal::debug_to_terminal;
use crossterm::{
    event::{self, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use std::io::{self, stdout, Stdout};

pub enum UserAction {
    Quit,
    TogglePopup,
    None,
}

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

pub fn draw_map(frame: &mut Frame, map: &Map, base: &mut Base, _show_popup: bool) {
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

    let instructions = Line::from(" Tab: menu - q: exit ".red());

    let text = Paragraph::new(styled_map_str).block(
        Block::default()
            .title(" Carte ")
            .title_bottom(instructions.centered())
            .borders(Borders::ALL),
    );

    let area = frame.size();
    frame.render_widget(text, area);

    if _show_popup {
        let area = centered_rect(60, 20, frame.size());

        let inventory = base.get_inventory();

        debug_to_terminal(&format!(
            "🎯 Inventaire actuel de la base : {:?}",
            inventory
        ));

        let block = Block::default()
            .title(" Gestion de la base ")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black).fg(Color::White));

        let content = format!(
            "Minerals : {}\nEnergy : {}\nPlans: {}",
            inventory.0, inventory.1, inventory.2
        );

        let text = Paragraph::new(content).block(block);

        frame.render_widget(Clear, area);
        frame.render_widget(text, area);
    }
}

pub fn handle_input() -> UserAction {
    if event::poll(std::time::Duration::from_millis(100)).unwrap() {
        if let event::Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Char('q') => return UserAction::Quit,
                KeyCode::Tab => return UserAction::TogglePopup,
                _ => {}
            }
        }
    }
    UserAction::None
}
