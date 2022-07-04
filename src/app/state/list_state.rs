use std::io::Stdout;

use tui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::{
    app::{
        actions::{Action, Actions},
        ui::check_size,
        App, AppReturn, GlobalState,
    },
    inputs::keys::Key,
};

use super::State;

//TODO: Too complicated for first app. Instead of doing this, just have
//a mapping of keys to action enum. Then, in do_action, do stuff based on that
//action enum. Simplify this later, don't do it first
pub struct ListState {
    actions: Actions,
    selected: u32,
}
impl ListState {
    pub fn new() -> Box<Self> {
        let actions = vec![Action::Quit, Action::SelectNext, Action::SelectPrev].into();
        return Box::new(Self {
            actions,
            selected: 0,
        });
    }
}
impl State for ListState {
    fn do_action(&mut self, key: Key, app: &GlobalState) -> AppReturn {
        if let Some(action) = self.actions.find(key) {
            match action {
                Action::Quit => AppReturn::Exit,
                Action::SelectNext => AppReturn::Continue,
                _ => AppReturn::Continue,
            }
        } else {
            return AppReturn::Continue;
        }
    }
    fn draw(&self, rect: &mut Frame<tui::backend::CrosstermBackend<Stdout>>, app: &App) {
        let size = rect.size();
        check_size(&size);
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(50)].as_ref())
            .split(size);
        let title = Paragraph::new("TODO List")
            .style(Style::default().fg(Color::LightCyan))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::White))
                    .border_type(BorderType::Plain),
            );

        rect.render_widget(title.clone(), chunks[0]);
        let body_content: Vec<Span> = app
            .global_state
            .list
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let mut cursor = " ";
                if i == self.selected.try_into().unwrap() {
                    cursor = ">";
                }
                return Span::styled(
                    String::from(cursor) + &item.content.clone(),
                    Style::default(),
                );
            })
            .collect();
        let body_paragraph = Paragraph::new(Spans::from(body_content)).block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default())
                .border_type(BorderType::Plain),
        );

        rect.render_widget(body_paragraph, chunks[1]);
    }
}
