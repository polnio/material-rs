use super::Widget;
use crate::Renderer;

pub struct Label<'a> {
    pub text: &'a str,
}

impl<'a> Widget for Label<'a> {
    fn render<R: Renderer>(&self, renderer: &mut R) {
        renderer.draw_text(self.text, Default::default());
    }
}
