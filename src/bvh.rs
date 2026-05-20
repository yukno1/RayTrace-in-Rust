use std::sync::Arc;

use crate::{
    aabb::AABB,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    interval::Interval,
    ray::Ray,
    utils::rand_usize,
};

// Bounding Volume Hierarchies
pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(objects: &mut [Arc<dyn Hittable>]) -> Self {
        let axis = rand_usize(0, 2);

        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };

        let (left, right): (Arc<dyn Hittable>, Arc<dyn Hittable>) = match objects.len() {
            1 => (Arc::clone(&objects[0]), Arc::new(NoHit)),
            2 => (Arc::clone(&objects[0]), Arc::clone(&objects[1])),
            _ => {
                objects.sort_by(|a, b| comparator(a, b));
                let mid = objects.len() / 2;
                (
                    Arc::new(Self::new(&mut objects[..mid])),
                    Arc::new(Self::new(&mut objects[mid..])),
                )
            }
        };

        let bbox = AABB::from_boxes(left.bounding_box(), right.bounding_box());
        Self { left, right, bbox }
    }

    pub fn from_hittable_list(mut list: HittableList) -> Self {
        Self::new(&mut list.objects)
    }

    fn box_compare(
        a: &Arc<dyn Hittable>,
        b: &Arc<dyn Hittable>,
        axis_index: usize,
    ) -> std::cmp::Ordering {
        let a_axis_interval = a.bounding_box().axis_interval(axis_index);
        let b_axis_interval = b.bounding_box().axis_interval(axis_index);
        a_axis_interval
            .min
            .partial_cmp(&b_axis_interval.min)
            .unwrap_or(std::cmp::Ordering::Equal)
    }

    fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord<'_>> {
        if !self.bbox.is_hit(r, ray_t) {
            return None;
        }
        let hit_left = self.left.hit(r, ray_t);
        let right_tmax = hit_left.as_ref().map_or(ray_t.max, |rec| rec.t);
        let hit_right = self.right.hit(r, Interval::new(ray_t.min, right_tmax));
        // hit_right, if it exists, is guaranteed closer (or equal), so prefer it
        hit_right.or(hit_left)
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

struct NoHit;

impl Hittable for NoHit {
    fn hit(&self, _r: &Ray, _ray_t: Interval) -> Option<HitRecord<'_>> {
        None
    }
    fn bounding_box(&self) -> AABB {
        AABB::default() // or however you represent an empty/degenerate AABB
    }
}
