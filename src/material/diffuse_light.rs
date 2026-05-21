use std::sync::Arc;

use crate::{
    color::Color,
    material::Material,
    texture::{SolidColor, Texture},
    vec3::Point3,
};

pub struct DiffuseLight {
    tex: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn from_color(emit: Color) -> Self {
        Self::from_tex(Arc::new(SolidColor::new(emit)))
    }

    pub fn from_tex(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, u: f64, v: f64, p: Point3) -> Color {
        self.tex.value(u, v, p)
    }
}
