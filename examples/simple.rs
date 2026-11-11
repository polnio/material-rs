use material_ui::prelude::*;
use std::io::Write as _;

fn main() {
    let mut buf = vec![0x2B; 800 * 600 * 3];
    let mut display = PpmScreen::new((800, 600), &mut buf);

    let mut app = material_ui::App::new();
    app.draw(&mut display, |ui| {
        ui.button("Hello, World!");
    });

    display.flush().unwrap();
}

struct PpmScreen<'a> {
    size: Size,
    data: &'a mut [u8],
}

impl<'a> PpmScreen<'a> {
    fn new(size: impl Into<Size>, data: &'a mut [u8]) -> Self {
        Self {
            size: size.into(),
            data,
        }
    }
    fn flush(&mut self) -> std::io::Result<()> {
        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();
        stdout.write_all(b"P6\n")?;
        stdout.write_all(self.size.width().to_string().as_bytes())?;
        stdout.write_all(b" ")?;
        stdout.write_all(self.size.height().to_string().as_bytes())?;
        stdout.write_all(b"\n255\n")?;
        stdout.write_all(self.data)?;
        stdout.flush()?;
        Ok(())
    }
}

impl DrawTarget for PpmScreen<'_> {
    #[allow(clippy::identity_op)]
    fn draw_pixel(&mut self, point: impl Into<Point>, color: Color) {
        if color.a == 0 {
            return;
        }
        let point = point.into();
        let x = point.x();
        let y = point.y();
        let w = self.size.width();
        let h = self.size.height();
        if x < 0 || x as u32 > w || y < 0 || y as u32 > h {
            return;
        }
        let i = y as usize * w as usize + x as usize;

        self.data[i * 3 + 0] = blend_color(color.r, self.data[i * 3 + 0], color.a);
        self.data[i * 3 + 1] = blend_color(color.g, self.data[i * 3 + 1], color.a);
        self.data[i * 3 + 2] = blend_color(color.b, self.data[i * 3 + 2], color.a);
    }
}

fn blend_color(src: u8, dst: u8, a: u8) -> u8 {
    ((a as u32 * src as u32 + (255 - a as u32) * dst as u32) / 255) as u8
}
