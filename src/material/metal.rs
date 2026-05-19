use crate::{color::Color, hittable::HitRecord, material::Material, ray::Ray, vec3::Vec3};

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut reflected = r_in.direction.reflect(rec.normal);
        reflected = reflected.unit_vec3() + (self.fuzz * Vec3::rand_unit_vec3());

        let scattered = Ray::new_with_time(rec.p, reflected, r_in.time);
        let attenuation = self.albedo;
        if scattered.direction * rec.normal > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
