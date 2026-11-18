use crate::{ColorPalette, DrawTarget, geometry::Rectangle, text::TextCtx};

pub struct App {
    pub(crate) color_palette: ColorPalette,
    pub(crate) text_ctx: TextCtx,
}
pub struct Ui<'a, D: DrawTarget> {
    pub(crate) app: &'a mut App,
    pub(crate) draw_target: &'a mut D,
    pub(crate) current_rect: Rectangle,
}

impl App {
    pub fn new() -> Self {
        Self {
            color_palette: ColorPalette::default_const(),
            text_ctx: TextCtx::new(),
        }
    }

    pub fn draw<'a, D: DrawTarget>(
        &'a mut self,
        target: &'a mut D,
        f: impl FnOnce(&mut Ui<'a, D>),
    ) {
        let size = target.size();
        let mut ui = Ui {
            app: self,
            draw_target: target,
            current_rect: Rectangle::new((0, 0).into(), size),
        };
        f(&mut ui);
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
