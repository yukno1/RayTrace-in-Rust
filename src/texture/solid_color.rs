use crate::{color::Color, texture::Texture, vec3::Point3};

pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    #[allow(dead_code)]
    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        Self::new(Color::new(r, g, b))
    }
}

#[allow(unused)]
impl Texture for SolidColor {
    #[inline]
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        self.albedo
    }
}
