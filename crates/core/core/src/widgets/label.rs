use super::Widget;
use crate::{geometry::Color, Renderer};

pub struct Label<'a> {
    pub text: &'a str,
}

impl<'a> Widget for Label<'a> {
    fn render<R: Renderer>(&self, renderer: &mut R) {
        renderer.draw_text(self.text, Default::default(), Color::WHITE);
    }
}
