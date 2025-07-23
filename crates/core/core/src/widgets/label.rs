use crate::geometry::Color;
use crate::geometry::Rect;
use crate::renderer::Renderer;
use crate::widgets::Widget;
use taffy::prelude::*;

pub struct Label<'a> {
    pub text: &'a str,

    layout_id: NodeId,
}

impl<'a> Label<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            layout_id: NodeId::from(0usize),
        }
    }
}

impl<'a> Widget for Label<'a> {
    fn layout<R: Renderer>(&mut self, renderer: &mut R) -> NodeId {
        let (w, h) = renderer.text_size(self.text);
        self.layout_id = renderer
            .inner()
            .taffy
            .new_leaf(Style {
                size: Size {
                    width: Dimension::from_length(w as f32),
                    height: Dimension::from_length(h as f32),
                },
                ..Default::default()
            })
            .unwrap();
        self.layout_id
    }
    fn render<R: Renderer>(&self, renderer: &mut R) {
        let node = renderer.inner().taffy.layout(self.layout_id).unwrap();
        let rect = Rect::from_layout(node);
        renderer.draw_text(self.text, rect.position(), Color::WHITE);
    }
}
