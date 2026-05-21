#![allow(non_snake_case)]
use std::sync::Arc;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    vec3::{Point3, Vec3},
};

pub struct Quad {
    Q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Arc<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    D: f64,
}

impl Quad {
    pub fn new(Q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
        let n = u.cross(v);
        let normal = n.unit_vec3();
        let D = normal * Q;
        let w = n / (n * n);

        let mut tmp = Self {
            Q,
            u,
            v,
            w,
            mat,
            bbox: AABB::default(),
            normal,
            D,
        };
        tmp.set_bounding_box();
        tmp
    }

    pub fn is_interior(a: f64, b: f64) -> Option<(f64, f64)> {
        let unit_interval = Interval::new(0.0, 1.0);
        // Given the hit point in plane coordinates, return false if it is outside the
        // primitive, otherwise set the hit record UV coordinates and return true.

        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return None;
        }

        return Some((a, b));
    }

    fn set_bounding_box(&mut self) {
        // Compute the bounding box of all 4 vertices.
        let bbox_diag1 = AABB::from_2_points(self.Q, self.Q + self.u + self.v);
        let bbox_diag2 = AABB::from_2_points(self.Q + self.u, self.Q + self.v);
        self.bbox = AABB::from_boxes(bbox_diag1, bbox_diag2);
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &crate::ray::Ray, ray_t: crate::interval::Interval) -> Option<HitRecord<'_>> {
        let denom = self.normal * r.direction;

        // No hit if the ray is parallel to the plane.
        if denom.abs() < 1e-8 {
            return None;
        }

        // Return false if the hit point parameter t is outside the ray interval.
        let t = (self.D - self.normal * r.origin) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        // Determine if the hit point lies within the planar shape using its plane coordinates.
        let intersection = r.at(t);
        let planar_hitpt_vec3 = intersection - self.Q;
        let alpha = self.w * planar_hitpt_vec3.cross(self.v);
        let beta = self.w * self.u.cross(planar_hitpt_vec3);

        match Self::is_interior(alpha, beta) {
            None => return None,
            Some((u, v)) => {
                // Ray hits the 2D shape; set the rest of the hit record and return true.
                let mut rec = HitRecord::new(r, t, intersection, self.normal, &*self.mat);
                rec.u = u;
                rec.v = v;
                Some(rec)
            }
        }
    }
    fn bounding_box(&self) -> AABB {
        return self.bbox;
    }
}
