use crate::geometry::{Point, Size};
use crate::{Color, DrawTarget};
use cosmic_text::{Attrs, Buffer, FontSystem, Shaping, SwashCache, fontdb::Source};

const ROBOTO: &[u8] = include_bytes!("../../../resources/Roboto/static/Roboto-Medium.ttf");

pub struct TextCtx {
    pub(crate) font_system: FontSystem,
    pub(crate) swash_cache: SwashCache,
}

impl TextCtx {
    pub fn new() -> Self {
        Self {
            font_system: FontSystem::new_with_fonts([Source::Binary(alloc::sync::Arc::new(
                ROBOTO,
            ))]),
            swash_cache: SwashCache::new(),
        }
    }
}

pub struct Text<'a> {
    buffer: Buffer,
    ctx: &'a mut TextCtx,
}

impl<'a> Text<'a> {
    pub fn new(
        ctx: &'a mut TextCtx,
        text: &str,
        max_width: Option<u32>,
        max_height: Option<u32>,
    ) -> Self {
        let mut buffer = Buffer::new(&mut ctx.font_system, cosmic_text::Metrics::new(16.0, 20.0));
        buffer.set_size(max_width.map(|w| w as f32), max_height.map(|h| h as f32));
        buffer.set_text(text, &Attrs::new(), Shaping::Advanced, None);
        buffer.shape_until_scroll(&mut ctx.font_system, false);
        Self { buffer, ctx }
    }
    pub fn measure(&self) -> Size {
        let mut width = 0f32;
        let mut height = 0f32;
        for run in self.buffer.layout_runs() {
            width = f32::max(width, run.line_w);
            height += run.line_height
        }
        (width as u32, height as u32).into()
    }
    pub fn draw(&mut self, point: Point, color: Color, target: &mut impl DrawTarget) {
        self.buffer.draw(
            &mut self.ctx.font_system,
            &mut self.ctx.swash_cache,
            color.into(),
            |x, y, w, h, c| {
                crate::primitive::rectangle(
                    target,
                    (x + point.x(), y + point.y(), w, h).into(),
                    c.into(),
                );
            },
        );
    }
}
