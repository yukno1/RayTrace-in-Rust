use crate::{color::Color, texture::Texture, vec3::Point3};

use super::perlin::Perlin;

pub struct NoiseTexture {
    noise: Perlin,
}

impl Default for NoiseTexture {
    fn default() -> Self {
        Self {
            noise: Perlin::new(),
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color {
        self.noise.noise(p) * Color::new(1.0, 1.0, 1.0)
    }
}
