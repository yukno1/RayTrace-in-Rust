use crate::{
    color::Color, interval::Interval, rtw_image::RtwImage, texture::Texture, vec3::Point3,
};

pub struct ImageTexture {
    image: RtwImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        Self {
            image: RtwImage::from_file(filename),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Point3) -> Color {
        // If we have no texture data, then return solid cyan as a debugging aid.
        if self.image.height() <= 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = Interval::new(0.0, 1.0).clamp(u);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v); // Flip V to image coordinates

        let i = (u * self.image.width() as f64) as u32;
        let j = (v * self.image.height() as f64) as u32;
        let pixel = self.image.pixel_data(i, j);

        let color_scale = 1.0 / 255.0;
        Color::new(
            pixel[0] as f64 * color_scale,
            pixel[1] as f64 * color_scale,
            pixel[2] as f64 * color_scale,
        )
    }
}
