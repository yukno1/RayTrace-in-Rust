use crate::{color::Color, hittable::HitRecord, materials::Material, ray::Ray, utils::rand_f64};

pub struct Dielectric {
    // Refractive index in vaccum or ait, or the ratio of the material's refractive index over the refractive index of enclosing media
    refraction_idx: f64,
}

impl Dielectric {
    pub fn new(refraction_idx: f64) -> Self {
        Self { refraction_idx }
    }

    fn reflectance(&self, cos: f64, ri: f64) -> f64 {
        // use Schlick's approximation for reflectance
        let mut r0 = (1.0 - ri) / (1.0 + ri);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_idx
        } else {
            self.refraction_idx
        };

        let unit_direction = r_in.direction.unit_vec3();
        let cos_theta = (-unit_direction * rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || (self.reflectance(cos_theta, ri) > rand_f64()) {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, ri)
        };

        let scattered = Ray::new_with_time(rec.p, direction, r_in.time);
        Some((attenuation, scattered))
    }
}
