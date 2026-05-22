use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    utils::degrees_to_radians,
    vec3::{Point3, Vec3},
};

pub struct RotateY {
    object: Box<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}

impl RotateY {
    pub fn rotate_y(object: impl Hittable + 'static, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox = object.bounding_box();

        let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }
        bbox = AABB::from_2_points(min, max);
        Self {
            object: Box::new(object),
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        // Transform the ray from world space to object space.
        let origin = Point3::new(
            (self.cos_theta * r.origin.x) - (self.sin_theta * r.origin.z),
            r.origin.y,
            (self.sin_theta * r.origin.x) + (self.cos_theta * r.origin.z),
        );
        let direction = Point3::new(
            (self.cos_theta * r.direction.x) - (self.sin_theta * r.direction.z),
            r.direction.y,
            (self.sin_theta * r.direction.x) + (self.cos_theta * r.direction.z),
        );

        let rotated_ray = Ray::new_with_time(origin, direction, r.time);

        // Determine whether an intersection exists in object space (and if so, where).
        match self.object.hit(&rotated_ray, ray_t) {
            None => None,

            Some(mut rec) => {
                // Transform the intersection from object space back to world space.
                rec.p = Point3::new(
                    (self.cos_theta * rec.p.x) + (self.sin_theta * rec.p.z),
                    rec.p.y,
                    (-self.sin_theta * rec.p.x) + (self.cos_theta * rec.p.z),
                );
                rec.normal = Point3::new(
                    (self.cos_theta * rec.normal.x) + (self.sin_theta * rec.normal.z),
                    rec.normal.y,
                    (-self.sin_theta * rec.normal.x) + (self.cos_theta * rec.normal.z),
                );
                Some(rec)
            }
        }
    }

    fn bounding_box(&self) -> crate::aabb::AABB {
        self.object.bounding_box()
    }
}
