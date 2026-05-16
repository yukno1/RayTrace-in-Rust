mod color;
mod hittable;
mod ray;
mod sphere;
mod vec3;

use crate::hittable::Hittable;
use crate::sphere::Sphere;
use crate::{color::*, ray::Ray, vec3::*};

// fn hit_sphere(centre: Point3, radius: f64, r: &Ray) -> f64 {
//     let oc = centre - r.origin;
//     let a = r.direction * r.direction;
//     let h = r.direction * oc;
//     let c = oc * oc - radius * radius;
//     let discriminant = h * h - a * c;

//     if discriminant < 0.0 {
//         return -1.0;
//     } else {
//         return (h - discriminant.sqrt()) / a;
//     }
// }

fn ray_color(ray: Ray) -> Color {
    let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    let rec = sphere.hit(&ray, 0.0, 100.0);
    match rec {
        Some(rec) => {
            let t = rec.t;
            // let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, &ray);

            // let n = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vec3();
            return 0.5 * Color::new(rec.normal.x + 1.0, rec.normal.y + 1.0, rec.normal.z + 1.0);
        }
        None => {
            let unit_direction = ray.direction.unit_vec3();
            let a = 0.5 * (unit_direction.y + 1.0);
            (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let mut image_height = (image_width as f64 / aspect_ratio) as usize;
    image_height = if image_height < 1 { 1 } else { image_height };

    // camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let camera_centre = Point3::zero();

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width;
    let pixel_delta_v = viewport_v / image_height;

    let viewport_upper_left =
        camera_centre - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel100_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let out = std::io::stdout();

    // render
    println!("P3\n{image_width} {image_height}\n255");

    for j in 0..image_height {
        eprintln!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let pixel_centre =
                pixel100_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_centre - camera_centre;
            let r = Ray::new(camera_centre, ray_direction);
            let pixel_color = ray_color(r);

            write_color(&out, pixel_color); // interior mutability of Stdout, so out no need to be mut
        }
    }
    eprintln!("\rDone");
}
