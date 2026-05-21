use crate::{
    color::{Color, write_color},
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    utils::{degrees_to_radians, rand_f64},
    vec3::{Point3, Vec3},
};

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,  // 1.0
    pub image_width: usize, // 400
    pub samples_per_pixel: usize,
    pub max_depth: usize,
    pub background: Color,

    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,          // camera-relative "up" direction
    pub defocus_angle: f64, // variation angle of rays through each pixel
    pub focus_dist: f64,    // distance from camera lookfrom point to plane of perfect focus

    // private
    image_height: usize,      // rendered image height
    pixel_samples_scale: f64, // color scale factor for a sum of pixel samples
    centre: Point3,
    pixel00_loc: Point3, // location of pixel 0,0
    pixel_delta_u: Vec3, // offset to pixel to the right
    pixel_delta_v: Vec3, // offset to pixel below
    u: Vec3,             // camera frame basis vectors
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3, // defocus disk horizontal radius
    defocus_disk_v: Vec3, // defocus disk vertical radius
}

impl Camera {
    // construct with default values
    pub fn new() -> Self {
        Self {
            // init pub
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,

            ..Default::default()
        }
    }

    pub fn init(&mut self) {
        // image
        let image_height = (self.image_width as f64 / self.aspect_ratio) as usize;
        self.image_height = if image_height < 1 { 1 } else { image_height };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.centre = self.lookfrom;

        // Determine viewport dimensions
        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * self.aspect_ratio;

        // calculate the u, v, w unit basis vectors for the camera coordinate frame
        self.w = (self.lookfrom - self.lookat).unit_vec3();
        self.u = self.vup.cross(self.w).unit_vec3();
        self.v = self.w.cross(self.u).unit_vec3();

        // calculate the vectors across the horizontal and vertical viewport edges
        let viewport_u = viewport_width * self.u; // vector across viewport horizontal edge
        let viewport_v = viewport_height * -self.v; // vector down viewport vertical edge

        // calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width;
        self.pixel_delta_v = viewport_v / self.image_height;

        // calculate the location of the upper left pixel
        let viewport_upper_left =
            self.centre - self.focus_dist * self.w - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.0);
        self.defocus_disk_u = defocus_radius * self.u;
        self.defocus_disk_v = defocus_radius * self.v;
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
                    pixel_color += self.ray_color(&r, self.max_depth, world);
                }

                write_color(&out, self.pixel_samples_scale * pixel_color); // interior mutability of Stdout, so out no need to be mut
            }
        }
        eprintln!("\rDone");
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        // construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.centre
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = rand_f64();
        Ray::new_with_time(ray_origin, ray_direction, ray_time)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(rand_f64(), rand_f64(), 0.0)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        // returns a random point in the camera defocus disk.
        let p = Vec3::rand_in_unit_disk();
        self.centre + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    fn ray_color(&self, ray: &Ray, depth: usize, world: &impl Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        // If the ray hits nothing, return the background color.
        // 1e-3 to avoid shadow acne
        match world.hit(ray, Interval::new(1e-3, f64::INFINITY)) {
            Some(rec) => {
                let color_from_emission = rec.mat.emitted(rec.u, rec.v, rec.p);
                if let Some((attenuation, scattered)) = rec.mat.scatter(ray, &rec) {
                    let color_from_scatter =
                        attenuation * self.ray_color(&scattered, depth - 1, world);
                    return color_from_emission + color_from_scatter;
                }
                return color_from_emission;
            }
            None => {
                // let unit_direction = ray.direction.unit_vec3();
                // let a = 0.5 * (unit_direction.y + 1.0);
                // (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
                self.background
            }
        }
    }
}
