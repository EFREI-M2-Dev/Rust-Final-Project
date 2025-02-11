use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{text::Line, widgets::Paragraph, style::Stylize};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::symbols::border;
use ratatui::text::Text;
use ratatui::widgets::Block;

#[derive(Debug)]
pub struct Home {
    pub exit: bool,
    options: Vec<&'static str>,
    selected_index: usize,
}

impl Home {

    pub fn new() -> Self {
        Self {
            exit: false,
            options: vec!["Lune", "Astéroïde", "Planète"],
            selected_index: 0,
        }
    }

    pub fn draw(&self, frame: &mut ratatui::Frame) {
        let area = frame.area();

        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(40),
                Constraint::Percentage(20),
            ])
            .split(area);

        let border_block = Block::bordered().border_set(border::THICK);
        frame.render_widget(border_block, area);

        let title = Line::from(" Bienvenue sur EREEA ").bold();
        let instructions = Line::from(" Esc pour quitter ".red());

        let block = Block::bordered().title(title.centered()).title_bottom(instructions.centered());
        frame.render_widget(block, frame.area());

        let description = Paragraph::new(Text::from(vec![
            Line::from("Salut ! Commence par choisir un type de planète :)"),
            Line::from("Utilise les flèches 'haut' et 'bas' si tu veux naviguer entre les différentes options"),
        ]))
            .centered();

        frame.render_widget(description, layout[1]);

        let options: Vec<Line> = self.options.iter()
            .enumerate()
            .map(|(i, &option)| {
                if i == self.selected_index {
                    Line::from(format!("> {}", option).yellow().bold())
                } else {
                    Line::from(format!("  {}", option))
                }
            })
            .collect();

        let menu = Paragraph::new(options).centered();

        frame.render_widget(menu, layout[2]);
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<&'static str> {
        match key_event.code {
            KeyCode::Esc => {
                self.exit = true
            }
            KeyCode::Up => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            KeyCode::Down => {
                if self.selected_index < self.options.len() - 1 {
                    self.selected_index += 1;
                }
            }
            KeyCode::Enter => return Some(self.options[self.selected_index]),
            _ => {}
        }
        None
    }

    pub fn should_exit(&self) -> bool {
        self.exit
    }
}