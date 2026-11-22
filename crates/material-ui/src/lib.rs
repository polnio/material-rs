#![no_std]

extern crate alloc;

pub mod component;
pub mod geometry;
pub mod util;

mod color;
mod text;

pub mod prelude {
    pub use crate::{
        Color, CursorState, DrawTarget,
        geometry::{Point, Size},
    };
}

mod app;
mod primitive;

pub use app::App;
pub use color::{Color, ColorPalette};

pub(crate) use app::Ui;

use crate::geometry::{Point, Size};

pub trait DrawTarget {
    fn size(&self) -> Size;
    fn cursor_pos(&self) -> Option<Point>;
    fn cursor_state(&self) -> CursorState;

    fn clear(&mut self, color: Color);
    fn draw_pixel(&mut self, point: impl Into<Point>, color: Color);
    fn flush(&mut self);
}

bitflags::bitflags! {
    #[derive(Clone, Copy)]
    pub struct CursorState: u8 {
        const LEFT_CLICK = 1 << 0;
    }
}

impl CursorState {
    fn left_click(&self) -> bool {
        self.intersects(Self::LEFT_CLICK)
    }
}
