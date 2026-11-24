use crate::geometry::{Rectangle, Size};
use crate::{Color, DrawTarget};

const BUTTON_HEIGHT: u32 = 56;
const BUTTON_PADDING: u32 = 24;
const BUTTON_HOVERED_LAYER: Color = Color::from_rgba(0xFFFFFF14); // White with opacity 0.08
const BUTTON_PRESSED_LAYER: Color = Color::from_rgba(0xFFFFFF1A); // White with opacity 0.1

pub struct Button {
    hovered: bool,
    clicked: bool,
}

impl<'a, D: DrawTarget> crate::Ui<'a, D> {
    pub fn button(&mut self, label: &str) -> Button {
        let mut text = crate::text::Text::new(
            &mut self.app.text_ctx,
            label,
            Some(self.current_rect.size().width()),
            Some(BUTTON_HEIGHT),
        );

        let txt_size = text.measure();
        let btn_size = Size::new(txt_size.width() + 2 * BUTTON_PADDING, BUTTON_HEIGHT);
        let btn_rect = Rectangle::new(self.current_rect.origin(), btn_size);
        let btn_radii = [
            BUTTON_HEIGHT / 2,
            BUTTON_HEIGHT / 2,
            BUTTON_HEIGHT / 2,
            BUTTON_HEIGHT / 2,
        ];

        let rr = crate::primitive::rounded_rectangle(
            self.draw_target,
            btn_rect,
            btn_radii,
            self.app.color_palette.primary,
        );

        let hovered = self.draw_target.cursor_pos().is_some_and(|p| rr.is_in(p));
        let clicked = hovered && self.draw_target.cursor_state().left_click();
        let pressed = hovered && self.draw_target.cursor_state().left_down();

        if pressed {
            crate::primitive::rounded_rectangle(
                self.draw_target,
                btn_rect,
                btn_radii,
                BUTTON_PRESSED_LAYER,
            );
        } else if hovered {
            crate::primitive::rounded_rectangle(
                self.draw_target,
                btn_rect,
                btn_radii,
                BUTTON_HOVERED_LAYER,
            );
        }

        text.draw(
            (
                self.current_rect.origin().x() + BUTTON_PADDING as i32,
                self.current_rect.origin().y()
                    + (BUTTON_HEIGHT as i32 - txt_size.height() as i32) / 2,
            )
                .into(),
            self.app.color_palette.primary_text,
            self.draw_target,
        );

        *self.current_rect.origin_mut().y_mut() += BUTTON_HEIGHT as i32;
        {
            let h = self.current_rect.size_mut().height_mut();
            *h = h.saturating_sub(BUTTON_HEIGHT);
        }

        Button { hovered, clicked }
    }
}

impl Button {
    #[inline]
    pub fn hovered(&self) -> bool {
        self.hovered
    }
    #[inline]
    pub fn clicked(&self) -> bool {
        self.clicked
    }
}
