use crate::geometry::Color;

#[derive(Debug)]
pub enum Profile {
    Light,
    Dark,
}

#[derive(Debug)]
pub struct Theme {
    inner: material_colors::theme::Theme,
    profile: Profile,
}

impl Theme {
    pub fn new_light(color: Color) -> Self {
        Self {
            inner: material_colors::theme::ThemeBuilder::with_source(color.into()).build(),
            profile: Profile::Light,
        }
    }
    pub fn new_dark(color: Color) -> Self {
        Self {
            inner: material_colors::theme::ThemeBuilder::with_source(color.into()).build(),
            profile: Profile::Dark,
        }
    }
    #[inline]
    pub fn scheme(&self) -> &material_colors::scheme::Scheme {
        match self.profile {
            Profile::Light => &self.inner.schemes.light,
            Profile::Dark => &self.inner.schemes.dark,
        }
    }
}
