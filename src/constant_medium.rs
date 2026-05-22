use std::sync::Arc;

use crate::{
    color::Color,
    hittable::{HitRecord, Hittable},
    interval::{Interval, UNIVERSE_INTERVAL},
    material::{Material, isotropic::Isotropic},
    ray::Ray,
    texture::Texture,
    utils::rand_f64,
    vec3::Vec3,
};

pub struct ConstantMedium {
    boundary: Box<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn from_color(boundary: impl Hittable + 'static, density: f64, albedo: Color) -> Self {
        Self {
            boundary: Box::new(boundary),
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::from_color(albedo)),
        }
    }

    pub fn from_tex(
        boundary: impl Hittable + 'static,
        density: f64,
        tex: Arc<dyn Texture>,
    ) -> Self {
        Self {
            boundary: Box::new(boundary),
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::from_tex(tex)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        match self.boundary.hit(r, UNIVERSE_INTERVAL) {
            None => return None,
            Some(mut rec1) => {
                match self
                    .boundary
                    .hit(r, Interval::new(rec1.t + 1e-4, f64::INFINITY))
                {
                    None => return None,
                    Some(mut rec2) => {
                        if rec1.t < ray_t.min {
                            rec1.t = ray_t.min;
                        }
                        if rec2.t > ray_t.max {
                            rec2.t = ray_t.max;
                        }
                        if rec1.t >= rec2.t {
                            return None;
                        }
                        if rec1.t < 0.0 {
                            rec1.t = 0.0;
                        }

                        let ray_length = r.direction.len();
                        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                        let hit_distance = self.neg_inv_density * rand_f64().ln();
                        if hit_distance > distance_inside_boundary {
                            return None;
                        }

                        let t = rec1.t + hit_distance / ray_length;
                        let p = r.at(t);
                        let rec = HitRecord::new(
                            r,
                            t,
                            p,
                            Vec3::new(1.0, 0.0, 0.0),
                            &*self.phase_function,
                        );
                        Some(rec)
                    }
                }
            }
        }
    }

    fn bounding_box(&self) -> crate::aabb::AABB {
        self.boundary.bounding_box()
    }
}
