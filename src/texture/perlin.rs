use crate::{
    utils::{rand_f64, rand_usize},
    vec3::Point3,
};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    randfloat: [f64; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut perlin = Self::default();
        for i in 0..POINT_COUNT {
            perlin.randfloat[i] = rand_f64();
        }
        Self::perlin_generate_perm(&mut perlin.perm_x);
        Self::perlin_generate_perm(&mut perlin.perm_y);
        Self::perlin_generate_perm(&mut perlin.perm_z);
        perlin
    }

    pub fn noise(&self, p: Point3) -> f64 {
        // rust 不会像 c++ 那样通过补码将负数回绕到 [0，255]
        let i = ((4.0 * p.x).floor() as isize & 255) as usize;
        let j = ((4.0 * p.y).floor() as isize & 255) as usize;
        let k = ((4.0 * p.z).floor() as isize & 255) as usize;

        self.randfloat[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }

    fn perlin_generate_perm(p: &mut [usize; POINT_COUNT]) {
        for i in 0..POINT_COUNT {
            p[i] = i;
        }
        Self::permute(p, POINT_COUNT);
    }

    fn permute(p: &mut [usize; POINT_COUNT], n: usize) {
        for i in (0..n).rev() {
            let target = rand_usize(0, i);
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self {
            randfloat: [0.0; POINT_COUNT],
            perm_x: [0; POINT_COUNT],
            perm_y: [0; POINT_COUNT],
            perm_z: [0; POINT_COUNT],
        }
    }
}
