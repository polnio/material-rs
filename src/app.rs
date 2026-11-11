use crate::{ColorPalette, DrawTarget, text::TextCtx};

pub struct App {
    pub(crate) color_palette: ColorPalette,
    pub(crate) text_ctx: TextCtx,
}
pub struct Ui<'a, D: DrawTarget> {
    pub(crate) app: &'a mut App,
    pub(crate) draw_target: &'a mut D,
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
        let mut ui = Ui {
            app: self,
            draw_target: target,
        };
        f(&mut ui);
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
