use bytemuck::{Pod, Zeroable};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Pod, Zeroable)]
#[repr(C)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Pod, Zeroable)]
#[repr(C)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Pod, Zeroable)]
#[repr(C)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}
impl Rect {
    pub fn position(&self) -> Point {
        Point {
            x: self.x,
            y: self.y,
        }
    }
    pub fn size(&self) -> Size {
        Size {
            width: self.width,
            height: self.height,
        }
    }
    pub const fn from_layout(layout: &taffy::Layout) -> Self {
        Self {
            x: layout.location.x as u32,
            y: layout.location.y as u32,
            width: layout.size.width as u32,
            height: layout.size.height as u32,
        }
    }

    pub fn contains(&self, point: Point) -> bool {
        self.x <= point.x
            && self.y <= point.y
            && self.x + self.width >= point.x
            && self.y + self.height >= point.y
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Pod, Zeroable)]
#[repr(C)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Color {
    pub const BLACK: Self = Self::from_u32(0x000000FF);
    pub const WHITE: Self = Self::from_u32(0xFFFFFFFF);
    pub const RED: Self = Self::from_u32(0xFF0000FF);
    pub const GREEN: Self = Self::from_u32(0x00FF00FF);
    pub const BLUE: Self = Self::from_u32(0x0000FFFF);
    const fn from_u32(color: u32) -> Self {
        bytemuck::must_cast(color.to_be())
    }
}
impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Self::from_u32(value)
    }
}
