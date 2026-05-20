use crate::{color::Color, vec3::Point3};

pub mod solid_color;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

pub use solid_color::SolidColor;
