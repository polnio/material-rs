use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, event, execute};
use material_core::geometry::{Color, Point, Rect};
use material_core::renderer::{Renderer, RendererInner};
use material_core::widgets::{IntoWidget, Widget};
use material_core::Theme;
use std::io::{Stdout, Write};

#[derive(Debug)]
pub struct TerminalRenderer {
    inner: RendererInner,
    stdout: Stdout,
}
impl TerminalRenderer {
    pub fn new(theme: Theme) -> Self {
        Self {
            inner: RendererInner::new(theme),
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
    fn inner(&mut self) -> &mut RendererInner {
        &mut self.inner
    }
    fn text_size(&self, text: &str) -> (u32, u32) {
        (text.len() as u32, 1)
    }
    fn draw_rect(&mut self, rect: Rect, color: Color, radius: u32) {
        let mut pos = rect.position();
        for _ in 0..rect.height {
            self.move_cursor(rect.position());
            println!("{}", "░".repeat(rect.width as usize));
            pos.y += 1;
        }
    }
    fn draw_text(&mut self, text: &str, pos: Point, color: Color) {
        self.move_cursor(pos);
        println!("{}", text);
    }
    fn render(&mut self, widget: impl IntoWidget) {
        enable_raw_mode().expect("Failed to enable raw mode");
        execute!(self.stdout, terminal::EnterAlternateScreen, cursor::Hide)
            .expect("Failed to hide cursor");
        let mut widget = widget.into_widget();
        self.compute_layout(&mut widget);
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
