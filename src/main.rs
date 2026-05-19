mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

use crate::{
    camera::Camera,
    color::Color,
    hittable_list::HittableList,
    material::{Dielectric, Lambertian, Material, Metal},
    sphere::Sphere,
    utils::{rand_f64, rand_f64_range},
    vec3::{Point3, Vec3},
};

fn main() {
    // world
    let mut world: HittableList<'static> = HittableList::new();

    let mat_ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        mat_ground,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand_f64();
            let centre = Point3::new(
                a as f64 + 0.9 * rand_f64(),
                0.2,
                b as f64 + 0.9 * rand_f64(),
            );

            if (centre - Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::from(Vec3::rand_vec3()) * Color::from(Vec3::rand_vec3());
                    let mat = Lambertian::new(albedo);
                    world.add(Sphere::new(centre, 0.2, mat));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::from(Vec3::rand_range_vec3(0.5, 1.0));
                    let fuzz = rand_f64_range(0.0, 0.5);
                    let mat = Metal::new(albedo, fuzz);
                    world.add(Sphere::new(centre, 0.2, mat));
                } else {
                    // glass
                    let mat = Dielectric::new(1.5);
                    world.add(Sphere::new(centre, 0.2, mat));
                };
            }
        }
    }

    let mat1 = Dielectric::new(1.5);
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1));

    let mat2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2));

    let mat3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3));

    let mut camera = Camera::new();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.samples_per_pixel = 500;
    camera.max_depth = 50;

    camera.vfov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;
    camera.init();

    camera.render(&world);
}

// fn main() {
//     // world
//     let mut world: HittableList<'static> = HittableList::new();

//     let mat_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
//     let mat_centre = Lambertian::new(Color::new(0.1, 0.2, 0.5));
//     let mat_left = Dielectric::new(1.5);
//     let mat_bubble = Dielectric::new(1.0 / 1.5);
//     let mat_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

//     world.add(Sphere::new(
//         Point3::new(0.0, -100.5, -1.0),
//         100.0,
//         mat_ground,
//     ));
//     world.add(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, mat_centre));
//     world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, mat_left));
//     world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, mat_bubble));
//     world.add(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, mat_right));

//     let mut camera = Camera::new();
//     camera.aspect_ratio = 16.0 / 9.0;
//     camera.image_width = 400;
//     camera.samples_per_pixel = 100;
//     camera.max_depth = 50;

//     camera.vfov = 20.0;
//     camera.lookfrom = Point3::new(-2.0, 2.0, 1.0);
//     camera.lookat = Point3::new(0.0, 0.0, -1.0);
//     camera.vup = Vec3::new(0.0, 1.0, 0.0);

//     camera.defocus_angle = 10.0;
//     camera.focus_dist = 3.4;
//     camera.init();

//     camera.render(&world);
// }
