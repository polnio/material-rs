use crate::geometry::{Color, Point, Rect};
use crate::widgets::IntoWidget;

pub trait Renderer {
    fn draw_text(&mut self, text: &str, pos: Point, color: Color);
    fn draw_rect(&mut self, rect: Rect, color: Color);
    fn render(&mut self, widget: impl IntoWidget);
}
