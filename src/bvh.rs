use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    interval::Interval,
    ray::Ray,
};

pub struct BVH_node {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    bbox: AABB,
}

impl BVH_node {
    pub fn new() -> Self {
        todo!()
    }

    pub fn from_hittableList(list: HittableList) -> Self {
        todo!()
    }
}

impl Hittable for BVH_node {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        todo!()
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
