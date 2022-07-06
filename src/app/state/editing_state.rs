use std::io::Stdout;

use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::{
    app::{
        actions::edit_action::{EditAction, EditActions},
        ui::check_size,
        App, AppReturn, GlobalState, Transition,
    },
    inputs::keys::Key,
};

use super::{State, StateReturn};

pub struct EditingState {
    actions: EditActions,
}
impl EditingState {
    pub fn new() -> Box<Self> {
        let actions = vec![EditAction::Quit, EditAction::List].into();
        return Box::new(Self { actions });
    }
}
impl State for EditingState {
    fn do_action(&mut self, key: Key, _app: &GlobalState) -> StateReturn {
        if let Some(action) = self.actions.find(key) {
            match action {
                EditAction::Quit => StateReturn {
                    transition: None,
                    app_return: AppReturn::Exit,
                },
                EditAction::List => StateReturn {
                    transition: Some(Transition::List),
                    app_return: AppReturn::Continue,
                },
                // _ => StateReturn {
                //     transition: None,
                //     app_return: AppReturn::Continue,
                // },
            }
        } else {
            return StateReturn {
                transition: None,
                app_return: AppReturn::Continue,
            };
        }
    }
    fn draw(&self, rect: &mut Frame<tui::backend::CrosstermBackend<Stdout>>, _app: &App) {
        let size = rect.size();
        check_size(&size);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(50)].as_ref())
            .split(size);
        let title = Paragraph::new("Editing")
            .style(Style::default().fg(Color::LightCyan))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .border_type(BorderType::Plain),
            );

        rect.render_widget(title.clone(), chunks[0]);
        let body_paragraph = Paragraph::new(Span::from("Testing Edit")).block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default())
                .border_type(BorderType::Plain),
        );

        rect.render_widget(body_paragraph, chunks[1]);
    }
}
