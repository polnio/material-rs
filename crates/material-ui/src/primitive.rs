use crate::geometry::{Point, Rectangle};
use crate::{Color, DrawTarget};

pub fn rectangle(target: &mut impl DrawTarget, rect: Rectangle, color: Color) {
    let x = rect.origin().x();
    let y = rect.origin().y();
    let w = rect.size().width();
    let h = rect.size().height();
    for y in y..y + h as i32 {
        for x in x..x + w as i32 {
            target.draw_pixel((x, y), color);
        }
    }
}

pub struct RoundedRectangle {
    rect: Rectangle,
    radii: [u32; 4],
}

impl RoundedRectangle {
    pub fn new(rect: Rectangle, radii: [u32; 4]) -> Self {
        let w = rect.size().width();
        let h = rect.size().height();
        let [tl, tr, bl, br] = radii;
        let tl = tl.min(w / 2).min(h / 2);
        let tr = tr.min(w / 2).min(h / 2);
        let bl = bl.min(w / 2).min(h / 2);
        let br = br.min(w / 2).min(h / 2);
        Self {
            rect,
            radii: [tl, tr, bl, br],
        }
    }
    pub fn is_in(&self, point: Point) -> bool {
        let x = point.x();
        let y = point.y();
        let rx = self.rect.origin().x();
        let ry = self.rect.origin().y();
        let w = self.rect.size().width();
        let h = self.rect.size().height();

        if x < 0 || y < 0 {
            // TODO: Not implemented
            return false;
        }
        if x < rx || y < ry || x > rx + w as i32 || y > ry + h as i32 {
            return false;
        }

        let x = (x - rx) as u32;
        let y = (y - ry) as u32;
        let [tl, tr, bl, br] = self.radii;
        if x < tl && y < tl && sqd(tl - x, tl - y) > tl * tl {
            return false;
        }
        if x + tr > w && y < tr && sqd(x + tr - w, tr - y) > tr * tr {
            return false;
        }
        if x < bl && y + bl > h && sqd(bl - x, y + bl - h) > bl * bl {
            return false;
        }
        if x + br > w && y + br > h && sqd(x + br - w, y + br - h) > br * br {
            return false;
        }
        true
    }

    fn coverage(&self, x: u32, y: u32) -> f32 {
        let [tl, tr, bl, br] = self.radii.map(|r| r as f32);
        let w = self.rect.size().width() as f32;
        let h = self.rect.size().height() as f32;

        // pixel center
        let px = x as f32 + 0.5;
        let py = y as f32 + 0.5;

        let corner = if px < tl && py < tl {
            Some((tl - px, tl - py, tl))
        } else if px + tr > w && py < tl {
            Some((px + tr - w, tl - py, tr))
        } else if px < bl && py + bl > h {
            Some((bl - px, py + bl - h, bl))
        } else if px + br > w && py + br > h {
            Some((px + br - w, py + br - h, br))
        } else {
            None
        };
        match corner {
            None => 1.0,
            Some((dx, dy, r)) => {
                let d = (dx * dx + dy * dy).sqrt() - r;
                (0.5 - d).clamp(0.0, 1.1)
            }
        }
    }
}

pub fn rounded_rectangle(
    target: &mut impl DrawTarget,
    rect: Rectangle,
    radii: [u32; 4],
    color: Color,
) -> RoundedRectangle {
    let rr = RoundedRectangle::new(rect, radii);
    let x = rect.origin().x();
    let y = rect.origin().y();
    let w = rect.size().width();
    let h = rect.size().height();
    for row in 0..h {
        for col in 0..w {
            let a = rr.coverage(col, row);
            if a > 0.0 {
                let mut c = color;
                c.a = (c.a as f32 * a).round() as u8;
                target.draw_pixel((x + col as i32, y + row as i32), c);
            }
        }
    }
    rr
}

#[inline]
fn sqd(dx: u32, dy: u32) -> u32 {
    dx * dx + dy * dy
}
