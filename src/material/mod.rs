pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Point3};

pub trait Material: Sync + Send {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        // (attenuation, scattered)
        None
    }
    fn emitted(&self, _u: f64, _v: f64, _p: Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

pub use dielectric::Dielectric;
pub use diffuse_light::DiffuseLight;
pub use lambertian::Lambertian;
pub use metal::Metal;
