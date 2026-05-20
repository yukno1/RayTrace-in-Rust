use crate::{
    color::Color,
    hittable::HitRecord,
    material::Material,
    ray::Ray,
    texture::{SolidColor, Texture},
    vec3::Vec3,
};
use std::sync::Arc;

pub struct Lambertian {
    tex: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }

    pub fn from_tex(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::rand_unit_vec3();

        // catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new_with_time(rec.p, scatter_direction, r_in.time);
        let attenuation = self.tex.value(rec.u, rec.v, rec.p);
        return Some((attenuation, scattered));
    }
}
