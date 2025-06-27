mod button;
mod label;

use crate::renderer::Renderer;
pub use button::Button;
pub use label::Label;
use taffy::NodeId;

pub trait Widget {
    fn layout<R: Renderer>(&mut self, renderer: &mut R) -> NodeId;
    fn render<R: Renderer>(&self, renderer: &mut R);
}

pub trait IntoWidget {
    type W: Widget;
    fn into_widget(self) -> Self::W;
}

impl<W: Widget> IntoWidget for W {
    type W = W;
    fn into_widget(self) -> Self::W {
        self
    }
}

impl<'a> IntoWidget for &'a str {
    type W = Label<'a>;
    fn into_widget(self) -> Self::W {
        Label { text: self }
    }
}
