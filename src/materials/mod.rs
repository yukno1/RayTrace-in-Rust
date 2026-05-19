pub mod dielectric;
pub mod lambertian;
pub mod metal;

use crate::{color::Color, hittable::HitRecord, ray::Ray};

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>; // (attenuation, scattered)
}

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;
