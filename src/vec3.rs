use std::ops;

use crate::utils::{rand_f64, rand_f64_range};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// type alias
#[allow(unused)]
pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    #[inline]
    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    #[inline]
    pub fn len_sq(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn cross(&self, rhs: Vec3) -> Self {
        Self {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    pub fn rand_vec3() -> Vec3 {
        Vec3::new(rand_f64(), rand_f64(), rand_f64())
    }

    pub fn rand_range_vec3(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            rand_f64_range(min, max),
            rand_f64_range(min, max),
            rand_f64_range(min, max),
        )
    }

    pub fn unit_vec3(&self) -> Vec3 {
        *self / self.len()
    }

    pub fn rand_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(rand_f64_range(-1.0, 1.0), rand_f64_range(-1.0, 1.0), 0.0);
            if p.len_sq() < 1.0 {
                return p;
            }
        }
    }

    pub fn rand_unit_vec3() -> Vec3 {
        loop {
            let p = Vec3::rand_range_vec3(-1.0, 1.0);
            if 1e-160 < p.len_sq() && p.len_sq() <= 1.0 {
                return p / p.len();
            }
        }
    }

    pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::rand_unit_vec3();

        if on_unit_sphere * normal > 0.0 {
            on_unit_sphere
        } else {
            -1.0 * on_unit_sphere
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        return self.x.abs() < s && self.y.abs() < s && self.z.abs() < s;
    }

    pub fn reflect(&self, n: Vec3) -> Self {
        return *self - 2.0 * n * (*self * n);
    }

    pub fn refract(&self, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-*self * n).min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * n);
        let r_out_parallel = -n * ((1.0 - r_out_perp.len_sq()).abs()).sqrt();

        return r_out_perp + r_out_parallel;
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Self::Output::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Self::Output::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = f64;
    fn mul(self, rhs: Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::Output::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Self::Output::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl ops::Div<usize> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: usize) -> Self::Output {
        Self::Output::new(
            self.x / rhs as f64,
            self.y / rhs as f64,
            self.z / rhs as f64,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(a + b, Vec3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_f64mul() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = 2.0;
        assert_eq!(b * a, Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(a * b, b * a);
    }
}
