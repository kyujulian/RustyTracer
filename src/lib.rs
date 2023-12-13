mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

use camera::Camera;
use material::{Dielectric, Lambertian, Metal};
use sphere::Sphere;

use hittable::HittableList;
use vec3::{Color, Point3, Vec3};

use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

pub fn final_scene() {
    let mut world: HittableList = HittableList::new();

    let ground_material = Rc::new(Lambertian::from(Color::from(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::from(
        Point3::from(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = utils::random_double();
            let center = Point3::from(
                a as f64 + 0.9 * utils::random_double(),
                0.2,
                b as f64 + 0.9 * utils::random_double(),
            );
            if (center - Point3::from(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn material::Material>;

                if choose_mat < 0.8 {
                    //Diffuse
                    let albedo = Color::random() * Color::random();

                    sphere_material = Rc::new(Lambertian::from(albedo));
                    world.add(Box::new(Sphere::from(
                        center,
                        0.2,
                        sphere_material.clone(),
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_in(0.5, 1.0);
                    let fuzz = utils::random_double_in(0.0, 0.5);
                    sphere_material = Rc::new(Metal::from(albedo, fuzz));
                    world.add(Box::new(Sphere::from(
                        center,
                        0.2,
                        sphere_material.clone(),
                    )));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::from(1.9));
    world.add(Box::new(Sphere::from(
        Point3::from(0.0, 1.0, 0.0),
        1.0,
        material1.clone(),
    )));

    let material2 = Rc::new(Lambertian::from(Color::from(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::from(
        Point3::from(-4.0, 1.0, 0.0),
        1.0,
        material2.clone(),
    )));

    let material3 = Rc::new(Metal::from(Color::from(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::from(
        Point3::from(4.0, 1.0, 0.0),
        1.0,
        material3.clone(),
    )));

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 1000;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::from(15.0, 2.0, 6.0);
    cam.lookat = Point3::from(0.0, 0.0, 0.0);
    cam.vup = Vec3::from(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world);
}

pub fn run() {
    //Materials
    let material_ground = Rc::new(Lambertian::from(Color::from(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::from(Color::from(0.1, 0.2, 0.5)));

    let material_right = Rc::new(Dielectric::from(1.5));
    let material_left = Rc::new(Metal::from(Color::from(0.8, 0.6, 0.2), 1.0));

    //World
    let mut world: HittableList = HittableList::new();
    world.add(Box::new(Sphere::from(
        Point3::from(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::from(
        Point3::from(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));

    world.add(Box::new(Sphere::from(
        Point3::from(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::from(
        Point3::from(1.0, 0.0, -1.0),
        -0.5,
        material_right,
    )));

    //
    //Camera
    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.samples_per_pixel = 20;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::from(10.0, 10.0, 1.0);
    cam.lookat = Point3::from(0.0, 0.0, -1.0);
    cam.vup = Vec3::from(0.0, 1.0, 0.0);

    cam.defocus_angle = 10.0;
    cam.focus_dist = 3.4;

    cam.render(&world);
}
