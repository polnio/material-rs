use crate::DrawTarget;
use crate::geometry::{Rectangle, Size};

const BUTTON_HEIGHT: u32 = 56;
const BUTTON_PADDING: u32 = 24;

impl<'a, D: DrawTarget> crate::Ui<'a, D> {
    pub fn button(&mut self, label: &str) {
        let mut text = crate::text::Text::new(
            &mut self.app.text_ctx,
            label,
            Some(self.current_rect.size().width()),
            Some(BUTTON_HEIGHT),
        );

        let txt_size = text.measure();
        let btn_size = Size::new(txt_size.width() + 2 * BUTTON_PADDING, BUTTON_HEIGHT);

        crate::primitive::rounded_rectangle(
            self.draw_target,
            Rectangle::new(self.current_rect.origin(), btn_size),
            [
                BUTTON_HEIGHT / 2,
                BUTTON_HEIGHT / 2,
                BUTTON_HEIGHT / 2,
                BUTTON_HEIGHT / 2,
            ],
            self.app.color_palette.primary,
        );

        text.draw(
            (
                self.current_rect.origin().x() + BUTTON_PADDING as i32,
                self.current_rect.origin().y()
                    + (BUTTON_HEIGHT as i32 - txt_size.height() as i32) / 2,
            )
                .into(),
            self.app.color_palette.white,
            self.draw_target,
        );

        *self.current_rect.origin_mut().y_mut() += BUTTON_HEIGHT as i32;
        *self.current_rect.size().width_mut() -= BUTTON_HEIGHT;
    }
}
