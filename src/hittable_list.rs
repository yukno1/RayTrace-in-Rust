use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

pub struct HittableList<'a> {
    objects: Vec<Box<dyn Hittable + 'a>>,
}

impl<'a> HittableList<'a> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    // takes ownership of object
    pub fn add(&mut self, object: impl Hittable + 'a) {
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
}
