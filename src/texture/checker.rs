use std::sync::Arc;

use crate::{
    color::Color,
    texture::{SolidColor, Texture},
};

pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: even,
            odd: odd,
        }
    }

    pub fn from_color(scale: f64, c1: Color, c2: Color) -> Self {
        Self::new(
            scale,
            Arc::new(SolidColor::new(c1)),
            Arc::new(SolidColor::new(c2)),
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: crate::vec3::Point3) -> crate::color::Color {
        let x_int = (self.inv_scale * p.x).floor() as isize;
        let y_int = (self.inv_scale * p.y).floor() as isize;
        let z_int = (self.inv_scale * p.z).floor() as isize;

        let is_even = (x_int + y_int + z_int) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
