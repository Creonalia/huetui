use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    style::{Color, Style},
    widgets::{Block, Borders},
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Char('q') => break Ok(()),
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget(
        Block::new()
            .borders(Borders::empty())
            .style(Style::default().bg(Color::White)),
        frame.area(),
    );
}
