use crate::{color::Color, hittable::HitRecord, materials::Material, ray::Ray, vec3::Vec3};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
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
        let attenuation = self.albedo;
        return Some((attenuation, scattered));
    }
}
