use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HitRecord {
    pub t: f64,
    pub p: Point3,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(r: &Ray, t: f64, p: Point3, normal: Vec3) -> Self {
        let mut rec = Self {
            t,
            p,
            normal,
            front_face: false,
        };
        rec.set_face_normal(r);
        rec
    }

    fn set_face_normal(&mut self, r: &Ray) {
        // Sets the hit record normal vec
        // assume normal is normalized
        self.front_face = r.direction * self.normal < 0.0;

        if !self.front_face {
            self.normal *= -1.0;
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
