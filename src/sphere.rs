use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Sphere<'a> {
    centre: Ray,
    radius: f64,
    mat: Box<dyn Material + 'a>,
}

impl<'a> Sphere<'a> {
    pub fn new(centre: Point3, radius: f64, mat: impl Material + 'a) -> Self {
        Self {
            centre: Ray::new(centre, Vec3::new(0.0, 0.0, 0.0)),
            radius: radius.max(0.0),
            mat: Box::new(mat),
        }
    }

    pub fn new_moving(
        centre1: Point3,
        centre2: Point3,
        radius: f64,
        mat: impl Material + 'a,
    ) -> Self {
        Self {
            centre: Ray::new(centre1, centre2 - centre1),
            radius: radius.max(0.0),
            mat: Box::new(mat),
        }
    }
}

impl<'a> Hittable for Sphere<'a> {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        let current_centre = self.centre.at(r.time);
        let oc = current_centre - r.origin;
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
            (p - current_centre) / self.radius,
            &*self.mat,
        ))
    }
}
