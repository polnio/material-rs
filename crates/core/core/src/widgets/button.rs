use super::{IntoWidget, Widget};
use crate::geometry::{Color, Rect};
use crate::Renderer;

pub struct Button<W: Widget> {
    child: W,
}
impl<W: Widget> Button<W> {
    pub fn new(child: impl IntoWidget<W = W>) -> Self {
        Self {
            child: child.into_widget(),
        }
    }
}

impl<W: Widget> Widget for Button<W> {
    fn render<R: Renderer>(&self, renderer: &mut R) {
        // TODO: Calculate layout
        renderer.draw_rect(
            Rect {
                x: 0,
                y: 0,
                width: 250,
                height: 40,
            },
            Color::GREEN,
        );
        self.child.render(renderer);
    }
}
