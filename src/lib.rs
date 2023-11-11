mod hittable;
mod ray;
mod vec3;

use ray::Ray;
use vec3::{Color, Point3, Vec3};

// const ASPECT_RATIO: i32 = 16.0 / 9.0;
// const IMAGE_WIDTH: i32 = 400;
//
// const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
// const VIEWPORT_HEIGHT: f64 = 2.0;
// const VIEWPORT_WIDTH: f64 =
//     ASPECT_RATIO * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);

pub fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(Point3::from(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let n = vec3::unit_vector(ray.at(t) - Vec3::from(0.0, 0.0, -1.0));
        return 0.5 * Color::from(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit_direction = vec3::unit_vector(ray.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - a) * Color::from(1.0, 1.0, 1.0)
        + a * Color::from(0.5, 0.7, 1.0);
}

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin() - center;
    let a = ray.direction().length_squared();

    let half_b = vec3::dot(&oc, &ray.direction());
    let c = oc.length_squared() - radius * radius;

    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}
pub fn run() {
    // Image

    let aspect_ratio = 16.0 / 9.0;

    let image_width = 400;

    let image_height = (image_width as f64 / aspect_ratio) as i32;

    let image_height = if image_height < 1 { 1 } else { image_height };

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

            let pixel_color = ray_color(&ray);
            vec3::write_color(std::io::stdout(), &pixel_color);
        }
    }

    log::info!("Done");
}
