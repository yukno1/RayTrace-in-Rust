use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

pub struct HittableList<'a> {
    objects: Vec<Box<dyn Hittable + 'a>>,
    bbox: AABB,
}

impl<'a> HittableList<'a> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: AABB::default(),
        }
    }

    // takes ownership of object
    pub fn add(&mut self, object: impl Hittable + 'a) {
        self.bbox = AABB::from_boxes(self.bbox, object.bounding_box().clone());
        self.objects.push(Box::new(object));
    }
}

impl<'a> Hittable for HittableList<'a> {
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
