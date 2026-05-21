use std::f64::consts::PI;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Sphere<'a> {
    centre: Ray,
    radius: f64,
    mat: Box<dyn Material + 'a>,
    bbox: AABB,
}

impl<'a> Sphere<'a> {
    pub fn new(static_centre: Point3, radius: f64, mat: impl Material + 'a) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        Self {
            centre: Ray::new(static_centre, Vec3::new(0.0, 0.0, 0.0)),
            radius: radius.max(0.0),
            mat: Box::new(mat),
            bbox: AABB::from_2_points(static_centre - rvec, static_centre + rvec),
        }
    }

    pub fn new_moving(
        centre1: Point3,
        centre2: Point3,
        radius: f64,
        mat: impl Material + 'a,
    ) -> Self {
        let centre = Ray::new(centre1, centre2 - centre1);
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = AABB::from_2_points(centre.at(0.0) - rvec, centre.at(0.0) + rvec);
        let box2 = AABB::from_2_points(centre.at(1.0) - rvec, centre.at(1.0) + rvec);
        Self {
            centre,
            radius: radius.max(0.0),
            mat: Box::new(mat),
            bbox: AABB::from_boxes(box1, box2),
        }
    }

    fn get_sphere_uv(p: Point3) -> (f64, f64) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
        let theta = f64::acos(-p.y);
        let phi = f64::atan2(-p.z, p.x);

        (phi / (2.0 * PI) + 0.5, theta / PI)
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
        let normal = (p - current_centre) / self.radius;
        let mut rec = HitRecord::new(r, root, p, normal, &*self.mat);
        (rec.u, rec.v) = Self::get_sphere_uv(rec.normal);
        Some(rec)
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
