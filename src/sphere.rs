use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub centre: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(centre: Vec3, radius: f64) -> Self {
        Self { centre, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.centre - r.origin;
        let a = r.direction * r.direction;
        let h = r.direction * oc;
        let c = oc * oc - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if !ray_t.contains(root) {
            root = (h + sqrtd) / a;

            if !ray_t.contains(root) {
                return None;
            }
        }
        let p = r.at(root);
        Some(HitRecord::new(r, root, p, (p - self.centre) / self.radius))
    }
}
