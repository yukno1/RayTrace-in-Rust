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

use std::f64::consts::PI;

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

    let R = (PI / 4.0).cos();

    let mat_left = Lambertian::new(Color::new(0.0, 0.0, 1.0));
    let mat_right = Lambertian::new(Color::new(1.0, 0.0, 0.0));

    world.add(Sphere::new(Point3::new(-R, 0.0, -1.0), R, mat_left));
    world.add(Sphere::new(Point3::new(R, 0.0, -1.0), R, mat_right));

    let mut camera = Camera::new();
    camera.max_depth = 50;

    camera.render(&world);
}
