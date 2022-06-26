use std::{
    io::{stdout, Error},
    time::Duration,
};

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, Terminal,
};

use crate::inputs::{events::Events, InputEvent};

use super::{state::AppState, App, AppReturn};

pub fn start_ui(app: &mut App) -> Result<(), Error> {
    let stdout = stdout();
    crossterm::terminal::enable_raw_mode().unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    //user event handling
    let tick_rate = Duration::from_millis(200);
    let events = Events::new(tick_rate);

    //main loop
    loop {
        terminal.draw(|rect| draw(rect, &app))?;
        let result = match events.next().unwrap() {
            InputEvent::Input(key) => app.do_action(key),
            InputEvent::Tick => app.update_on_tick(),
        };
        if result == AppReturn::Exit {
            break;
        }
    }
    terminal.clear()?;
    terminal.show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
pub fn draw<B>(rect: &mut Frame<B>, app: &App)
where
    B: tui::backend::Backend,
{
    let size = rect.size();
    check_size(&size);
    if app.state == AppState::List {
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
            .list
            .iter()
            .map(|item| Span::styled(item.content.clone(), Style::default()))
            .collect();
        let body_paragraph = Paragraph::new(Spans::from(body_content)).block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default())
                .border_type(BorderType::Plain),
        );

        // let body_chunks = Layout::default()
        //     .direction(Direction::Vertical)
        //     .split(chunks[1]);
        rect.render_widget(body_paragraph, chunks[1]);
    }
}

pub fn check_size(rect: &Rect) {
    if rect.width < 52 {
        panic!("Require width >= 52, (got {})", rect.width);
    }
}
