use material_ui::geometry::{Point, Size};
use material_ui::util::draw_blended_pixel;
use material_ui::{Color, DrawTarget};
use std::io::Write as _;

pub struct PpmScreen<'a> {
    size: Size,
    data: &'a mut [u8],
}

impl<'a> PpmScreen<'a> {
    pub fn new(size: impl Into<Size>, data: &'a mut [u8]) -> Self {
        Self {
            size: size.into(),
            data,
        }
    }
    pub fn flush(&mut self) -> std::io::Result<()> {
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
    fn size(&self) -> Size {
        self.size
    }
    fn clear(&mut self) {
        self.data.fill(0);
    }
    fn flush(&mut self) {
        let _ = self.flush();
    }
    #[allow(clippy::identity_op)]
    fn draw_pixel(&mut self, point: impl Into<Point>, color: Color) {
        draw_blended_pixel(self.data, self.size, point, color);
    }
}
