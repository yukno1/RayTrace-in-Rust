mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod materials;
mod ray;
mod sphere;
mod utils;
mod vec3;

use crate::{
    camera::Camera,
    color::Color,
    hittable_list::HittableList,
    materials::{Dielectric, Lambertian, Metal},
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
                    let centre2 = centre + Vec3::new(0.0, rand_f64_range(0.0, 0.5), 0.0);
                    world.add(Sphere::new_moving(centre, centre2, 0.2, mat));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::rand(0.5, 1.0);
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
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
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
