use std::{
    io::{stdout, Error},
    time::Duration,
};

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, Terminal,
};

pub fn start_ui() -> Result<(), Error> {
    let stdout = stdout();
    crossterm::terminal::enable_raw_mode().unwrap();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    //user event handling
    let _tick_rate = Duration::from_millis(200);

    //main loop
    loop {
        terminal.draw(|rect| draw(rect))?;
    }
    terminal.clear()?;
    terminal.show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
pub fn draw<B>(rect: &mut Frame<B>)
where
    B: tui::backend::Backend,
{
    let size = rect.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3)].as_ref())
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

    rect.render_widget(title, chunks[0])
}
