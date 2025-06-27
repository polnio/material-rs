use crate::geometry::Point;
use crate::widgets::IntoWidget;

pub trait Renderer {
    fn draw_text(&mut self, text: &str, pos: Point);
    fn render(&mut self, widget: impl IntoWidget);
}
