use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

use std::sync::Arc;

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new(object: impl Hittable + 'static) -> Self {
        let mut l = Self::default();
        l.add(object);
        l
    }

    // takes ownership of object
    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.bbox = AABB::from_boxes(self.bbox, object.bounding_box().clone());
        self.objects.push(Arc::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, mut ray_t: Interval) -> Option<HitRecord<'_>> {
        let mut temp: Option<HitRecord> = None;
        for obj in self.objects.iter() {
            if let Some(rec) = obj.hit(r, ray_t) {
                ray_t.max = rec.t;
                temp = Some(rec);
            }
        }
        temp
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self {
            objects: Vec::new(),
            bbox: AABB::default(),
        }
    }
}
