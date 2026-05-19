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
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
    vec3::Point3,
};

fn main() {
    // world
    let mut world: HittableList<'static> = HittableList::new();

    let mat_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let mat_centre = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let mat_left = Dielectric::new(1.5);
    let mat_bubble = Dielectric::new(1.0 / 1.5);
    let mat_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        mat_ground,
    ));
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, mat_centre));
    world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, mat_left));
    world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, mat_bubble));
    world.add(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, mat_right));

    let mut camera = Camera::new();
    camera.max_depth = 50;
    camera.render(&world);
}
