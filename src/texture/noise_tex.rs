use crate::{color::Color, texture::Texture, vec3::Point3};

use super::perlin::Perlin;

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::default(),
            scale,
        }
    }
}

impl Default for NoiseTexture {
    fn default() -> Self {
        Self {
            noise: Perlin::default(),
            scale: 1.0,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Color {
        self.noise.turb(p, 7) * Color::new(1.0, 1.0, 1.0)
    }
}
