use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    palette::Hsl,
    style::{Color, Style},
    widgets::{Block, Borders},
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let result = App::default().run(&mut terminal);
    ratatui::restore();
    result
}

#[derive(Debug)]
pub struct App {
    hsl: Hsl,
    exit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            hsl: Hsl::new(0.0, 0.0, 1.0),
            exit: false,
        }
    }
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(
            Block::new()
                .borders(Borders::empty())
                .style(Style::default().bg(Color::from_hsl(self.hsl))),
            frame.area(),
        );
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Char('q') => self.exit = true,
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }
}
