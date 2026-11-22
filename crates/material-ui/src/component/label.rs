use crate::DrawTarget;

impl<'a, D: DrawTarget> crate::Ui<'a, D> {
    pub fn label(&mut self, text: &str) {
        crate::text::Text::new(
            &mut self.app.text_ctx,
            text,
            Some(self.current_rect.size().width()),
            Some(self.current_rect.size().height()),
        )
        .draw(
            self.current_rect.origin(),
            self.app.color_palette.white,
            self.draw_target,
        );
    }
}
