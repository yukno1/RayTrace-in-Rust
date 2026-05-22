use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    vec3::Vec3,
};

pub struct Translate {
    object: Box<dyn Hittable>,
    offset: Vec3,
    bbox: AABB,
}

impl Translate {
    pub fn translate(object: impl Hittable + 'static, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            object: Box::new(object),
            offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        // Move the ray backwards by the offset
        let offset_r = Ray::new_with_time(r.origin - self.offset, r.direction, r.time);

        // Determine whether an intersection exists along the offset ray (and if so, where)
        match self.object.hit(&offset_r, ray_t) {
            Some(mut rec) => {
                // Move the intersection point forwards by the offset
                rec.p += self.offset;
                Some(rec)
            }
            None => None,
        }
    }

    fn bounding_box(&self) -> AABB {
        return self.bbox;
    }
}
