// Axis-Aligned Bounding Boxes

use crate::{
    hittable::Hitbox,
    interval::{EMPTY_INTERVAL, Interval, UNIVERSE_INTERVAL},
    ray::Ray,
    vec3::Point3,
};

#[derive(Clone, Copy)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

const EMPTY_AABB: AABB = AABB {
    x: EMPTY_INTERVAL,
    y: EMPTY_INTERVAL,
    z: EMPTY_INTERVAL,
};
const UNIVERSE_AABB: AABB = AABB {
    x: UNIVERSE_INTERVAL,
    y: UNIVERSE_INTERVAL,
    z: UNIVERSE_INTERVAL,
};

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut tmp = Self { x, y, z };
        tmp.pad_to_min();
        tmp
    }

    pub fn from_2_points(a: Point3, b: Point3) -> Self {
        // Treat the two points a and b as extrema for the bounding box, so we don't require a
        // particular minimum/maximum coordinate order.
        let x = Interval::new(a.x.min(b.x), a.x.max(b.x));
        let y = Interval::new(a.y.min(b.y), a.y.max(b.y));
        let z = Interval::new(a.z.min(b.z), a.z.max(b.z));
        let mut tmp = Self { x, y, z };
        tmp.pad_to_min();
        tmp
    }

    pub fn from_boxes(box0: AABB, box1: AABB) -> AABB {
        let x = Interval::from_intervals(box0.x, box1.x);
        let y = Interval::from_intervals(box0.y, box1.y);
        let z = Interval::from_intervals(box0.z, box1.z);

        Self::new(x, y, z)
    }

    pub fn axis_interval(&self, n: usize) -> Interval {
        match n {
            1 => self.y,
            2 => self.z,
            _ => self.x,
        }
    }

    pub fn is_hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        let ray_orig = r.origin;
        let ray_dir = r.direction;

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis];

            let t0 = (ax.min - ray_orig[axis]) * adinv;
            let t1 = (ax.max - ray_orig[axis]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }
            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    pub fn longest_axis(&self) -> usize {
        // Returns the index of the longest axis of the bounding box.

        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() { 0 } else { 2 }
        } else {
            if self.y.size() > self.z.size() { 1 } else { 2 }
        }
    }

    fn pad_to_min(&mut self) {
        // Adjust the AABB so that no side is narrower than some delta, padding if necessary.
        let delta = 1e-4;
        if (self.x.size() < delta) {
            self.x = self.x.expand(delta)
        };
        if (self.y.size() < delta) {
            self.y = self.y.expand(delta)
        };
        if (self.z.size() < delta) {
            self.z = self.z.expand(delta)
        };
    }
}

impl Default for AABB {
    fn default() -> Self {
        EMPTY_AABB
    }
}

impl Hitbox for AABB {
    fn is_hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        let ray_orig = r.origin;
        let ray_dir = r.direction;

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis];

            let t0 = (ax.min - ray_orig[axis]) * adinv;
            let t1 = (ax.max - ray_orig[axis]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }
            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    fn bounding_box(&self) -> Self {
        *self
    }
}
