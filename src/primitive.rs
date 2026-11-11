use crate::{Color, DrawTarget, geometry::Rectangle};

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

pub fn rounded_rectangle(
    target: &mut impl DrawTarget,
    rect: Rectangle,
    radii: [u32; 4],
    color: Color,
) {
    let x = rect.origin().x();
    let y = rect.origin().y();
    let w = rect.size().width();
    let h = rect.size().height();
    let [tl, tr, bl, br] = radii;
    let tl = tl.min(w / 2).min(h / 2);
    let tr = tr.min(w / 2).min(h / 2);
    let bl = bl.min(w / 2).min(h / 2);
    let br = br.min(w / 2).min(h / 2);
    for row in 0..h {
        for col in 0..w {
            if col < tl && row < tl && sqd(tl - col, tl - row) > tl * tl {
                continue;
            }
            if col + tr > w && row < tr && sqd(col + tr - w, tr - row) > tr * tr {
                continue;
            }
            if col < bl && row + bl > h && sqd(bl - col, row + bl - h) > bl * bl {
                continue;
            }
            if col + br > w && row + br > h && sqd(col + tr - w, row + bl - h) > br * br {
                continue;
            }

            target.draw_pixel((x + col as i32, y + row as i32), color);
        }
    }
}

#[inline]
fn sqd(dx: u32, dy: u32) -> u32 {
    dx * dx + dy * dy
}
