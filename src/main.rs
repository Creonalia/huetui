use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    palette::{ClampAssign, Hsl},
    style::{Color, Style},
    text::Line,
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
    show_footer: bool,
    exit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            hsl: Hsl::new(0.0, 0.0, 1.0),
            show_footer: true,
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
        let color = Color::from_hsl(self.hsl);
        let mut color_block = Block::new().style(Style::default().bg(color));
        if self.show_footer {
            color_block = color_block
                .borders(Borders::BOTTOM)
                .border_style(Style::reset().fg(color))
                .title_style(Style::reset())
                .title_bottom(
                    Line::from(format![
                        " Hue (-h/+l): {} | Saturation (-u/+i): {:.2} | Lightness (-j/+k) {:.2} ",
                        self.hsl.hue.into_degrees(),
                        self.hsl.saturation,
                        self.hsl.lightness,
                    ])
                    .centered(),
                )
                .title_bottom(Line::from(" Footer (?) ").left_aligned())
                .title_bottom(Line::from(" Quit (q) ").right_aligned());
        }

        frame.render_widget(color_block, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Char('q') => self.exit = true,
                    KeyCode::Char('?') => self.show_footer = !self.show_footer,

                    KeyCode::Char('j') => self.hsl.lightness -= 0.05,
                    KeyCode::Char('J') => self.hsl.lightness -= 0.01,
                    KeyCode::Char('k') => self.hsl.lightness += 0.05,
                    KeyCode::Char('K') => self.hsl.lightness += 0.01,

                    KeyCode::Char('h') => self.hsl.hue -= 5.0,
                    KeyCode::Char('H') => self.hsl.hue -= 1.0,
                    KeyCode::Char('l') => self.hsl.hue += 5.0,
                    KeyCode::Char('L') => self.hsl.hue += 1.0,

                    KeyCode::Char('u') => self.hsl.saturation -= 0.05,
                    KeyCode::Char('U') => self.hsl.saturation -= 0.01,
                    KeyCode::Char('i') => self.hsl.saturation += 0.05,
                    KeyCode::Char('I') => self.hsl.saturation += 0.01,

                    _ => {}
                }
                self.hsl.clamp_assign();
            }
            _ => {}
        }
        Ok(())
    }
}
