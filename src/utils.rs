#![allow(unused)]

use fastrand;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}

pub fn rand_f64() -> f64 {
    // range 0..1
    fastrand::f64()
}

pub fn rand_usize(min: usize, max: usize) -> usize {
    fastrand::usize(min..(max + 1))
}

pub fn rand_f64_range(min: f64, max: f64) -> f64 {
    min + (max - min) * rand_f64()
}
