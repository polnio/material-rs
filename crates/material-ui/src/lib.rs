#![no_std]

extern crate alloc;

pub mod component;
pub mod geometry;
mod text;

pub mod prelude {
    pub use crate::{
        Color, DrawTarget,
        geometry::{Point, Size},
    };
}

mod app;
mod primitive;

pub use app::App;

pub(crate) use app::Ui;

use crate::geometry::{Point, Size};

pub trait DrawTarget {
    fn draw_pixel(&mut self, point: impl Into<Point>, color: Color);
    fn size(&self) -> Size;
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct ColorPalette {
    pub primary: Color,
    pub white: Color,
}

impl ColorPalette {
    pub const fn default_const() -> Self {
        Self {
            primary: Color::from_rgba(0xEBABABFF),
            white: Color::from_rgba(0xEBEBEBFF),
        }
    }
}

impl Color {
    pub const fn from_rgba(hex: u32) -> Self {
        let [r, g, b, a] = hex.to_be_bytes();
        Self { r, g, b, a }
    }
    pub const fn from_argb(hex: u32) -> Self {
        let [a, r, g, b] = hex.to_be_bytes();
        Self { r, g, b, a }
    }
    pub const fn to_rgba(self) -> u32 {
        let Self { r, g, b, a } = self;
        u32::from_be_bytes([r, g, b, a])
    }
    pub const fn to_argb(self) -> u32 {
        let Self { r, g, b, a } = self;
        u32::from_be_bytes([a, r, g, b])
    }
}

impl From<Color> for cosmic_text::Color {
    fn from(val: Color) -> Self {
        cosmic_text::Color(val.to_argb())
    }
}

impl From<cosmic_text::Color> for Color {
    fn from(value: cosmic_text::Color) -> Self {
        Self::from_argb(value.0)
    }
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self::default_const()
    }
}
