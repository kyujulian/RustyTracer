mod camera;
mod hittable;
mod ray;
mod sphere;
mod vec3;
mod utils;

use camera::Camera;
use rand::prelude::*;
use sphere::Sphere;

use hittable::HittableList;
use vec3::Point3;

pub fn run() {
    //World
    let mut world: HittableList = HittableList::new();
    world.add(Box::new(Sphere::from(Point3::from(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::from(
        Point3::from(0.0, -100.5, -1.0),
        100.0,
    )));

    //Camera
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.samples_per_pixel = 100;
    cam.render(&world);
}
