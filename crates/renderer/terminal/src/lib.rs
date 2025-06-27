use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, event, execute};
use material_core::geometry::Point;
use material_core::widgets::{IntoWidget, Widget};
use material_core::Renderer;
use std::io::{Stdout, Write};

#[derive(Debug)]
pub struct TerminalRenderer {
    stdout: Stdout,
}
impl TerminalRenderer {
    pub fn new() -> Self {
        Self {
            stdout: std::io::stdout(),
        }
    }
    pub fn render(&mut self, widget: impl IntoWidget) {
        Renderer::render(self, widget);
    }
    fn move_cursor(&mut self, pos: Point) {
        execute!(self.stdout, cursor::MoveTo(pos.x as u16, pos.y as u16))
            .expect("Failed to move cursor");
    }
}

impl Renderer for TerminalRenderer {
    fn draw_text(&mut self, text: &str, pos: Point) {
        self.move_cursor(pos);
        println!("{}", text);
    }
    fn render(&mut self, widget: impl IntoWidget) {
        enable_raw_mode().expect("Failed to enable raw mode");
        execute!(self.stdout, terminal::EnterAlternateScreen, cursor::Hide)
            .expect("Failed to hide cursor");
        let widget = widget.into_widget();
        widget.render(self);
        self.stdout.flush().expect("Failed to flush stdout");
        loop {
            match event::read().expect("Failed to read event") {
                event::Event::Key(_) => {
                    break;
                }
                _ => {}
            };
        }
        execute!(self.stdout, terminal::LeaveAlternateScreen, cursor::Show)
            .expect("Failed to show cursor");
        disable_raw_mode().expect("Failed to disable raw mode");
    }
}
