use crate::Color;
use crate::geometry::{Point, Size};

pub fn draw_pixel(buffer: &mut [u8], size: Size, point: impl Into<Point>, color: Color) {
    if color.a == 0 {
        return;
    }
    let point = point.into();
    let x = point.x();
    let y = point.y();
    let w = size.width();
    let h = size.height();
    if x < 0 || x as u32 > w || y < 0 || y as u32 > h {
        return;
    }
    let i = y as usize * w as usize + x as usize;

    blend_color_mut(color, &mut buffer[i * 4..]);
    buffer[i * 4 + 3] = u8::MAX;
}

pub fn draw_blended_pixel(buffer: &mut [u8], size: Size, point: impl Into<Point>, color: Color) {
    if color.a == 0 {
        return;
    }
    let point = point.into();
    let x = point.x();
    let y = point.y();
    let w = size.width();
    let h = size.height();
    if x < 0 || x as u32 > w || y < 0 || y as u32 > h {
        return;
    }
    let i = y as usize * w as usize + x as usize;

    blend_color_mut(color, &mut buffer[i * 3..]);
}

pub fn blend_color(color: Color, current: [u8; 3]) -> [u8; 3] {
    let r = blend_color_part(color.r, current[0], color.a);
    let g = blend_color_part(color.g, current[1], color.a);
    let b = blend_color_part(color.b, current[2], color.a);
    [r, g, b]
}

pub fn blend_color_mut(color: Color, dst: &mut [u8]) {
    dst[0] = blend_color_part(color.r, dst[0], color.a);
    dst[1] = blend_color_part(color.g, dst[1], color.a);
    dst[2] = blend_color_part(color.b, dst[2], color.a);
}

pub fn blend_color_part(src: u8, dst: u8, a: u8) -> u8 {
    ((a as u32 * src as u32 + (255 - a as u32) * dst as u32) / 255) as u8
}
