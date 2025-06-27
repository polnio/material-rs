use crate::geometry::Color;
use crate::renderer::Renderer;
use crate::widgets::Widget;
use taffy::prelude::*;

pub struct Label<'a> {
    pub text: &'a str,
}

impl<'a> Widget for Label<'a> {
    fn layout<R: Renderer>(&mut self, renderer: &mut R) -> NodeId {
        let (w, h) = renderer.text_size(self.text);
        renderer
            .inner()
            .taffy
            .new_leaf(Style {
                size: Size {
                    width: Dimension::from_length(w as f32),
                    height: Dimension::from_length(h as f32),
                },
                ..Default::default()
            })
            .unwrap()
    }
    fn render<R: Renderer>(&self, renderer: &mut R) {
        renderer.draw_text(self.text, Default::default(), Color::WHITE);
    }
}
