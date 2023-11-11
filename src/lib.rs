mod hittable;
mod ray;
mod sphere;
mod vec3;

use sphere::Sphere;
use std::f64::INFINITY;

use hittable::{HitRecord, Hittable, HittableList};
use ray::Ray;
use vec3::{Color, Point3, Vec3};

pub fn ray_color<H: Hittable>(ray: &Ray, world: &H) -> Color {
    let mut rec = HitRecord::new();

    if world.hit(ray, hittable::Interval::from(0.0, INFINITY), &mut rec) {
        return 0.5 * (rec.normal + Color::from(1.0, 1.0, 1.0));
    }
    let unit_direction = vec3::unit_vector(ray.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - a) * Color::from(1.0, 1.0, 1.0)
        + a * Color::from(0.5, 0.7, 1.0);
}

pub fn run() {
    // Image

    let aspect_ratio = 16.0 / 9.0;

    let image_width = 400;

    // Calculate the image height, and ensure that it's at least 1
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let image_height = if image_height < 1 { 1 } else { image_height };

    //World
    let mut world: HittableList = HittableList::new();
    world.add(Box::new(Sphere::from(Point3::from(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::from(
        Point3::from(0.0, -100.5, -1.0),
        100.0,
    )));

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width =
        viewport_height * (image_width as f64 / image_height as f64);

    let camera_center = Point3::from(0.0, 0.0, 0.0);

    // Calculate the vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::from(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::from(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate the location of the upper left pixel.

    let viewport_upper_left = camera_center
        - Vec3::from(0.0, 0.0, focal_length)
        - viewport_u / 2.0
        - viewport_v / 2.0;

    let pixel00_loc =
        viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        log::info!(r"Scanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i * pixel_delta_u) + (j * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::from(camera_center, ray_direction);

            let pixel_color = ray_color(&ray, &world);
            vec3::write_color(std::io::stdout(), &pixel_color);
        }
    }

    log::info!("Done");
}
