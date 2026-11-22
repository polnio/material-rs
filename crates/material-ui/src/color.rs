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
    pub primary_text: Color,
    pub white: Color,
    pub background: Color,
}

impl ColorPalette {
    pub const fn default_const() -> Self {
        Self {
            primary: Color::from_rgba(0xEBABABFF),
            primary_text: Color::from_rgba(0x00000000),
            white: Color::from_rgba(0xEBEBEBFF),
            background: Color::from_rgba(0x333333FF),
        }
    }
    pub const fn catppuccin_mocha_blue() -> Self {
        Self {
            primary: Color::from_rgba(0x89B4FAFF),
            primary_text: Color::from_rgba(0x1E1E2EFF),
            white: Color::from_rgba(0xCDD6F4FF),
            background: Color::from_rgba(0x1E1E2EFF),
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
