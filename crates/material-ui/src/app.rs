use crate::geometry::{Point, Rectangle};
use crate::{ColorPalette, DrawTarget, text::TextCtx};

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
            color_palette: ColorPalette::catppuccin_mocha_blue(),
            text_ctx: TextCtx::new(),
        }
    }

    pub fn draw<D: DrawTarget>(&mut self, target: &mut D, f: impl for<'a> FnOnce(&mut Ui<'a, D>)) {
        let size = target.size();
        target.clear(self.color_palette.background);
        let mut ui = Ui {
            app: self,
            draw_target: target,
            current_rect: Rectangle::new(Point::new(0, 0), size),
        };
        f(&mut ui);
        target.flush();
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
