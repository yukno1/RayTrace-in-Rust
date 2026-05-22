use std::sync::Arc;

use crate::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    texture::{SolidColor, Texture},
    vec3::Vec3,
};

use super::Material;

pub struct Isotropic {
    tex: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn from_color(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }

    pub fn from_tex(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scattered = Ray::new_with_time(rec.p, Vec3::rand_unit_vec3(), r_in.time);
        let attenuation = self.tex.value(rec.u, rec.v, rec.p);

        Some((attenuation, scattered))
    }
}
