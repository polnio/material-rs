use derive_more::From;

#[derive(Clone, Copy, From)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, From)]
pub struct Size {
    width: u32,
    height: u32,
}

#[derive(Clone, Copy, From)]
pub struct Rectangle {
    origin: Point,
    size: Size,
}

impl Point {
    #[inline]
    pub const fn x(&self) -> i32 {
        self.x
    }
    #[inline]
    pub const fn y(&self) -> i32 {
        self.y
    }
}

impl Size {
    #[inline]
    pub const fn width(&self) -> u32 {
        self.width
    }
    #[inline]
    pub const fn height(&self) -> u32 {
        self.height
    }
}

impl Rectangle {
    #[inline]
    pub const fn origin(&self) -> Point {
        self.origin
    }
    #[inline]
    pub const fn size(&self) -> Size {
        self.size
    }
}

impl From<(i32, i32, u32, u32)> for Rectangle {
    fn from((x, y, w, h): (i32, i32, u32, u32)) -> Self {
        Self {
            origin: Point { x, y },
            size: Size {
                width: w,
                height: h,
            },
        }
    }
}
