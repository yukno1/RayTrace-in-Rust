use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere<'a> {
    centre: Vec3,
    radius: f64,
    mat: Box<dyn Material + 'a>,
}

impl<'a> Sphere<'a> {
    pub fn new(centre: Vec3, radius: f64, mat: impl Material + 'a) -> Self {
        Self {
            centre,
            radius,
            mat: Box::new(mat),
        }
    }
}

impl<'a> Hittable for Sphere<'a> {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
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
        Some(HitRecord::new(
            r,
            root,
            p,
            (p - self.centre) / self.radius,
            &*self.mat,
        ))
    }
}
