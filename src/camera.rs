use crate::color::{Color, write_color};
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: usize,
    pub samples_per_pixel: usize,

    image_height: usize,
    pixel_samples_scale: f64,
    centre: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    // construct with default values
    pub fn new() -> Self {
        // image
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 400;
        let mut image_height = (image_width as f64 / aspect_ratio) as usize;
        image_height = if image_height < 1 { 1 } else { image_height };

        let samples_per_pixel = 100;

        // camera
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;
        let centre = Point3::zero();

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / image_width;
        let pixel_delta_v = viewport_v / image_height;

        let viewport_upper_left =
            centre - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,

            image_height,
            pixel_samples_scale: 1.0 / (samples_per_pixel as f64),
            centre,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &impl Hittable) {
        let out = std::io::stdout();
        // render
        println!(
            "P3\n{image_width} {image_height}\n255",
            image_width = self.image_width,
            image_height = self.image_height
        );

        for j in 0..self.image_height {
            eprintln!("\rScanlines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, world);
                }

                // let pixel_centre = self.pixel00_loc
                //     + (i as f64 * self.pixel_delta_u)
                //     + (j as f64 * self.pixel_delta_v);
                // let ray_direction = pixel_centre - self.centre;
                // let r = Ray::new(self.centre, ray_direction);
                // let pixel_color = self.ray_color(&r, world);

                write_color(&out, self.pixel_samples_scale * pixel_color); // interior mutability of Stdout, so out no need to be mut
            }
        }
        eprintln!("\rDone");
    }

    fn initialize(&mut self) {}

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        // construct a ray originating from the camera and pointing at the pixel (i, j)
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);
        let ray_origin = self.centre;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(fastrand::f64(), fastrand::f64(), 0.0)
    }

    fn ray_color(&self, ray: &Ray, world: &impl Hittable) -> Color {
        match world.hit(ray, Interval::new(0.0, f64::INFINITY)) {
            Some(rec) => {
                return 0.5
                    * Color::new(rec.normal.x + 1.0, rec.normal.y + 1.0, rec.normal.z + 1.0);
            }
            None => {
                let unit_direction = ray.direction.unit_vec3();
                let a = 0.5 * (unit_direction.y + 1.0);
                (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
            }
        }
    }
}
