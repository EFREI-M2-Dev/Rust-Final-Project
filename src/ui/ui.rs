use crate::map::Map;
use crate::ui::centered_rect::centered_rect;
use crate::{map::base::Base, robot::RobotType};
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
    CreateSelectedRobot,
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

    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(0)])
        .split(area);

    frame.render_widget(text, chunks[0]);

    if _show_popup {
        /*  let progress = *base.research_progress.lock().unwrap() * 100.0;
        let progress_bar = BarChart::default()
            .block(Block::default().title("Progression de la Recherche").borders(Borders::ALL))
            .data(&[("Recherche", progress as u64)])
            .bar_style(Style::default().fg(Color::Green))
            .value_style(Style::default().fg(Color::White));

        frame.render_widget(progress_bar, chunks[1]); */

        let area = centered_rect(60, 20, frame.area());
        let inventory = base.get_inventory();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(4), Constraint::Length(7)])
            .split(area);

        let robot_counts = map.count_robots_by_type();

        let robot_types = vec![
            RobotType::Explorator,
            RobotType::Collector,
            RobotType::Scientist,
        ];

        let robot_info: Vec<String> = robot_types
            .iter()
            .map(|rt| {
                let count = robot_counts.get(rt).unwrap_or(&0);
                format!("{:?}: {} dispo(s)", rt, count)
            })
            .collect();

        let content = format!(
            "Minerals : {} | Energy : {} | Plans: {} \n{}",
            inventory.0,
            inventory.1,
            inventory.2,
            robot_info.join(" | ")
        );
        let block = Block::default()
            .title(" Gestion de la base ")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White));
        let text = Paragraph::new(content).block(block);

        let robot_types = vec![
            RobotType::Explorator,
            RobotType::Collector,
            RobotType::Scientist,
        ];

        let options: Vec<String> = robot_types
            .iter()
            .map(|rt| {
                let (m, e, p) = rt.cost();
                format!("{:?} (M:{}, E:{}, P:{})", rt, m, e, p)
            })
            .collect();

        let items: Vec<ListItem> = options
            .iter()
            .map(|opt| ListItem::new(opt.clone()))
            .collect();

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
            if key.kind == event::KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => return UserAction::Quit,
                    KeyCode::Tab => return UserAction::TogglePopup,
                    KeyCode::Up => return UserAction::MoveUp,
                    KeyCode::Down => return UserAction::MoveDown,
                    KeyCode::Enter => return UserAction::CreateSelectedRobot,
                    _ => {}
                }
            }
        }
    }
    UserAction::None
}
