use std::ops::*;

use crate::interval::Interval;
use crate::vec3::Vec3;

pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

impl From<Vec3> for Color {
    fn from(vec3: Vec3) -> Self {
        if vec3.len() == 1.0 {
            Self::new(vec3.x, vec3.y, vec3.z)
        } else {
            let v = vec3.unit_vec3();
            Self::new(v.x, v.y, v.z)
        }
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Self::Output {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self * rhs.r, self * rhs.g, self * rhs.b)
    }
}

const INTENSITY: Interval = Interval::new(0.0, 0.999);

pub fn write_color(mut out: impl std::io::Write, pixel_color: Color) {
    let rbyte = (256.0 * INTENSITY.clamp(pixel_color.r)) as usize;
    let gbyte = (256.0 * INTENSITY.clamp(pixel_color.g)) as usize;
    let bbyte = (256.0 * INTENSITY.clamp(pixel_color.b)) as usize;

    writeln!(out, "{rbyte} {gbyte} {bbyte}").unwrap(); // assume it works, otherwise panic
}
