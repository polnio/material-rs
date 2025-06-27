mod button;
mod label;

pub use button::Button;
pub use label::Label;

use crate::Renderer;

pub trait Widget {
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
