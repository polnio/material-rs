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
        self.is_in_inner((x - rx) as u32, (y - ry) as u32, self.radii)
    }

    fn is_in_inner(&self, x: u32, y: u32, radii: [u32; 4]) -> bool {
        let [tl, tr, bl, br] = radii;
        let w = self.rect.size().width();
        let h = self.rect.size().height();
        if x < tl && y < tl && sqd(tl - x, tl - y) > tl * tl {
            return false;
        }
        if x + tr > w && y < tr && sqd(x + tr - w, tr - y) > tr * tr {
            return false;
        }
        if x < bl && y + bl > h && sqd(bl - x, y + bl - h) > bl * bl {
            return false;
        }
        if x + br > w && y + br > h && sqd(x + tr - w, y + bl - h) > br * br {
            return false;
        }
        true
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
            if rr.is_in_inner(col, row, radii) {
                target.draw_pixel((x + col as i32, y + row as i32), color);
            }
        }
    }
    rr
}

#[inline]
fn sqd(dx: u32, dy: u32) -> u32 {
    dx * dx + dy * dy
}
