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
    MoveUp,
    MoveDown,
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

pub fn draw_map(
    frame: &mut Frame,
    map: &Map,
    base: &mut Base,
    _show_popup: bool,
    selected_index: usize,
) {
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

    let instructions = Line::from(" Tab: Menu - q: Quitter ".red());

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
            "[Base] \tInventaire actuel de la base : {:?}",
            inventory
        ));

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(4), Constraint::Length(7)])
            .split(area);

        let content = format!(
            "Minerals : {}\nEnergy : {}\nPlans: {}",
            inventory.0, inventory.1, inventory.2
        );
        let block = Block::default()
            .title(" Gestion de la base ")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White));
        let text = Paragraph::new(content).block(block);

        let options = vec!["Option 1", "Option 2", "Option 3"];
        let items: Vec<ListItem> = options.iter().map(|opt| ListItem::new(*opt)).collect();

        let menu = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Ajouter un nouveau module "),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::White)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("➤ ");

        let mut state = ListState::default();
        state.select(Some(selected_index));

        frame.render_widget(Clear, area);
        frame.render_widget(text, chunks[0]);
        frame.render_stateful_widget(menu, chunks[1], &mut state);
    }
}

pub fn handle_input() -> UserAction {
    if event::poll(std::time::Duration::from_millis(100)).unwrap() {
        if let event::Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Char('q') => return UserAction::Quit,
                KeyCode::Tab => return UserAction::TogglePopup,
                KeyCode::Up => return UserAction::MoveUp,
                KeyCode::Down => return UserAction::MoveDown,
                _ => {}
            }
        }
    }
    UserAction::None
}
