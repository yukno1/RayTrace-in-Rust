#![allow(unused)]

#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

pub const EMPTY_INTERVAL: Interval = Interval::new(f64::INFINITY, -f64::INFINITY);
pub const UNIVERSE_INTERVAL: Interval = Interval::new(-f64::INFINITY, f64::INFINITY);

impl Interval {
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn from_intervals(a: Interval, b: Interval) -> Self {
        // Create the interval tightly enclosing the two input intervals.
        let min = if a.min <= b.min { a.min } else { b.min };
        let max = if a.max >= b.max { a.max } else { b.max };

        Self::new(min, max)
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, other: &Self) -> bool {
        self.min <= other.min && other.max <= self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Self::new(self.min - padding, self.max + padding)
    }
}

impl Default for Interval {
    // Default interval is empty
    fn default() -> Self {
        EMPTY_INTERVAL
    }
}
