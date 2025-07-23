use crate::geometry::{Color, Rect};
use crate::renderer::Renderer;
use crate::widgets::{IntoWidget, Widget};
use taffy::prelude::*;

const BUTTON_HEIGHT: f32 = 40.0;
const BUTTON_HPADDING: f32 = 16.0;

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
            .new_with_children(
                Style {
                    size: Size {
                        width: Dimension::auto(),
                        height: Dimension::length(BUTTON_HEIGHT),
                    },
                    padding: taffy::Rect {
                        left: LengthPercentage::length(BUTTON_HPADDING),
                        right: LengthPercentage::length(BUTTON_HPADDING),
                        top: LengthPercentage::length(0.0),
                        bottom: LengthPercentage::length(0.0),
                    },
                    align_items: Some(AlignItems::Center),
                    ..Default::default()
                },
                &[child],
            )
            .unwrap();
        self.layout_id
    }
    fn render<R: Renderer>(&self, renderer: &mut R) {
        let node = renderer.inner().taffy.layout(self.layout_id).unwrap();
        let rect = Rect::from_layout(node);
        let color = if rect.contains(renderer.inner().cursor_pos) {
            Color::RED
        } else {
            Color::GREEN
        };
        renderer.draw_rect(rect, color, rect.height / 2);
        self.child.render(renderer);
    }
}
