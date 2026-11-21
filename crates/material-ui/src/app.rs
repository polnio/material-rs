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

    pub fn draw<D: DrawTarget>(&mut self, target: &mut D, f: impl for<'a> FnOnce(&mut Ui<'a, D>)) {
        let size = target.size();
        target.clear();
        let mut ui = Ui {
            app: self,
            draw_target: target,
            current_rect: Rectangle::new((0, 0).into(), size),
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
