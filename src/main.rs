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
    camera::Camera, color::Color, hittable_list::HittableList, material::Lambertian,
    sphere::Sphere, vec3::Point3,
};

fn main() {
    // world
    let mut world: HittableList<'static> = HittableList::new();
    world.add(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Lambertian::new(Color::new(0.8, 0.3, 0.3)),
    ));
    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Lambertian::new(Color::new(0.8, 0.3, 0.3)),
    ));

    let mut camera = Camera::new();
    camera.max_depth = 50;
    camera.render(&world);
}
