use crate::geometry::{Color, Rect};
use crate::renderer::Renderer;
use crate::widgets::{IntoWidget, Widget};
use taffy::prelude::*;

pub struct Button<W: Widget> {
    child: W,
    layout_id: NodeId,
}
impl<W: Widget> Button<W> {
    pub fn new(child: impl IntoWidget<W = W>) -> Self {
        Self {
            child: child.into_widget(),
            layout_id: NodeId::from(0usize),
        }
    }
}

impl<W: Widget> Widget for Button<W> {
    fn layout<R: Renderer>(&mut self, renderer: &mut R) -> NodeId {
        let child = self.child.layout(renderer);
        self.layout_id = renderer
            .inner()
            .taffy
            .new_with_children(Style::default(), &[child])
            .unwrap();
        self.layout_id
    }
    fn render<R: Renderer>(&self, renderer: &mut R) {
        let node = renderer.inner().taffy.layout(self.layout_id).unwrap();
        let rect = Rect::from_layout(node);
        renderer.draw_rect(rect, Color::GREEN);
        self.child.render(renderer);
    }
}
