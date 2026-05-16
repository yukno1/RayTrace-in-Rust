use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp: Option<HitRecord> = None;
        let mut closest = t_max;
        for obj in self.objects.iter() {
            if let Some(rec) = obj.hit(r, t_min, closest) {
                closest = rec.t;
                temp = Some(rec);
            }
        }
        temp
    }
}
