use crate::geometry::{Color, Point, Rect};
use crate::widgets::{IntoWidget, Widget};
use taffy::TaffyTree;

pub trait Renderer {
    fn inner(&mut self) -> &mut RendererInner;
    fn text_size(&self, text: &str) -> (u32, u32);
    fn draw_text(&mut self, text: &str, pos: Point, color: Color);
    fn draw_rect(&mut self, rect: Rect, color: Color);
    fn render(&mut self, widget: impl IntoWidget);

    fn compute_layout<W: Widget>(&mut self, widget: &mut W)
    where
        Self: Sized,
    {
        let root = widget.layout(self);
        let (w, h) = self.inner().surface_size;
        self.inner()
            .taffy
            .compute_layout(
                root,
                taffy::Size {
                    width: taffy::AvailableSpace::Definite(w as f32),
                    height: taffy::AvailableSpace::Definite(h as f32),
                },
            )
            .unwrap();
    }
}

#[derive(Debug, Clone, Default)]
pub struct RendererInner {
    pub(crate) taffy: TaffyTree,
    pub surface_size: (u32, u32),
}

impl RendererInner {
    pub fn new() -> Self {
        Self::default()
    }
}
