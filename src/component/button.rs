use crate::DrawTarget;

const BUTTON_HEIGHT: u32 = 56;
const BUTTON_PADDING: u32 = 24;

impl<'a, D: DrawTarget> crate::Ui<'a, D> {
    pub fn button(&mut self, label: &str) {
        let mut text =
            crate::text::Text::new(&mut self.app.text_ctx, label, None, Some(BUTTON_HEIGHT));

        let size = text.measure();

        crate::primitive::rounded_rectangle(
            self.draw_target,
            (0, 0, size.width() + 2 * BUTTON_PADDING, BUTTON_HEIGHT).into(),
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
                BUTTON_PADDING as i32,
                (BUTTON_HEIGHT as i32 - size.height() as i32) / 2,
            )
                .into(),
            self.app.color_palette.white,
            self.draw_target,
        );
    }
}
