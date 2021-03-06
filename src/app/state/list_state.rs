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
        actions::{ListAction, ListActions},
        ui::check_size,
        App, AppReturn, GlobalState, Transition,
    },
    inputs::keys::Key,
};

use super::{State, StateReturn};

//TODO: Too complicated for first app. Instead of doing this, just have
//a mapping of keys to action enum. Then, in do_action, do stuff based on that
//action enum. Simplify this later, don't do it first
pub struct ListState {
    actions: ListActions,
    selected: u32,
}
impl ListState {
    pub fn new() -> Box<Self> {
        let actions = vec![
            ListAction::Quit,
            ListAction::SelectNext,
            ListAction::SelectPrev,
        ]
        .into();
        return Box::new(Self {
            actions,
            selected: 0,
        });
    }
}
impl State for ListState {
    fn do_action(&mut self, key: Key, _app: &GlobalState) -> StateReturn {
        if let Some(action) = self.actions.find(key) {
            match action {
                ListAction::Quit => StateReturn {
                    transition: None,
                    app_return: AppReturn::Exit,
                },
                ListAction::SelectNext => {
                    self.selected = self.selected + 1;

                    return StateReturn {
                        transition: None,
                        app_return: AppReturn::Continue,
                    };
                }
                ListAction::Edit => StateReturn {
                    transition: Some(Transition::Editing),
                    app_return: AppReturn::Continue,
                },
                _ => StateReturn {
                    transition: None,
                    app_return: AppReturn::Continue,
                },
            }
        } else {
            return StateReturn {
                transition: None,
                app_return: AppReturn::Continue,
            };
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
        let body_content: Vec<Spans> = app
            .global_state
            .list
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let mut cursor = " ";
                if i == self.selected.try_into().unwrap() {
                    cursor = ">";
                }
                return Spans::from(Span::styled(
                    String::from(cursor) + &item.content.clone(),
                    Style::default(),
                ));
            })
            .collect();
        let body_paragraph = Paragraph::new(body_content).block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default())
                .border_type(BorderType::Plain),
        );

        rect.render_widget(body_paragraph, chunks[1]);
    }
}
