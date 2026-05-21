use crate::{
    utils::{rand_f64, rand_f64_range, rand_usize},
    vec3::{Point3, Vec3},
};

const POINT_COUNT: usize = 256;

pub struct Perlin {
    randvec: [Vec3; POINT_COUNT],
    perm_x: [usize; POINT_COUNT],
    perm_y: [usize; POINT_COUNT],
    perm_z: [usize; POINT_COUNT],
}

impl Perlin {
    // pub fn new() -> Self {
    //     let mut perlin = Self::default();
    //     for i in 0..POINT_COUNT {
    //         perlin.randfloat[i] = rand_f64();
    //     }
    //     Self::perlin_generate_perm(&mut perlin.perm_x);
    //     Self::perlin_generate_perm(&mut perlin.perm_y);
    //     Self::perlin_generate_perm(&mut perlin.perm_z);
    //     perlin
    // }

    pub fn noise(&self, p: Point3) -> f64 {
        let u = p.x - (p.x).floor();
        let v = p.y - (p.y).floor();
        let w = p.z - (p.z).floor();

        // rust 不会像 c++ 那样通过补码将负数回绕到 [0，255]
        let i = p.x.floor() as isize;
        let j = p.y.floor() as isize;
        let k = p.z.floor() as isize;
        let mut c = [[[Vec3::default(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.randvec[self.perm_x[((i + di as isize) & 255) as usize]
                        ^ self.perm_y[((j + dj as isize) & 255) as usize]
                        ^ self.perm_z[((k + dk as isize) & 255) as usize]];
                }
            }
        }

        Self::perlin_interp(c, u, v, w)
    }

    pub fn turb(&self, p: Point3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
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

    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                        * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                        * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                        * (c[i][j][k] * weight_v);
                }
            }
        }
        accum
    }
}

impl Default for Perlin {
    fn default() -> Self {
        let mut perlin = Self {
            randvec: [Vec3::default(); POINT_COUNT],
            perm_x: [0; POINT_COUNT],
            perm_y: [0; POINT_COUNT],
            perm_z: [0; POINT_COUNT],
        };
        for i in 0..POINT_COUNT {
            perlin.randvec[i] = Vec3::rand_vec3_range(-1.0, 1.0).unit_vec3();
        }
        Self::perlin_generate_perm(&mut perlin.perm_x);
        Self::perlin_generate_perm(&mut perlin.perm_y);
        Self::perlin_generate_perm(&mut perlin.perm_z);
        perlin
    }
}
