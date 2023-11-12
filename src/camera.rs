use crate::hittable;
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::ray::Ray;
use crate::vec3;
use crate::vec3::{Color, Point3, Vec3};

use std::f64::INFINITY;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    image_height: i32,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    center: Point3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            image_height: 225,
            pixel00_loc: Point3::new(),
            pixel_delta_u: Vec3::new(),
            pixel_delta_v: Vec3::new(),
            center: Point3::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.image_height =
            (self.image_width as f64 / self.aspect_ratio) as i32;

        if self.image_height < 1 {
            self.image_height = 1;
        }
        self.center = Point3::from(0.0, 0.0, 0.0);

        // Determine the viewport dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height
            * (self.image_width as f64 / self.image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = Vec3::from(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::from(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.

        let viewport_upper_left = self.center
            - Vec3::from(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;

        self.pixel00_loc = viewport_upper_left
            + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    pub fn render(&mut self, world: &HittableList) {
        self.initialize();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            log::info!(r"Scanlines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (i * self.pixel_delta_u)
                    + (j * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let ray = Ray::from(self.center, ray_direction);

                let pixel_color = self.ray_color(&ray, &world);
                vec3::write_color(std::io::stdout(), &pixel_color);
            }
        }

        log::info!("Done");
    }

    fn ray_color(&self, ray: &Ray, world: &HittableList) -> vec3::Color {
        let mut rec = HitRecord::new();

        if world.hit(ray, hittable::Interval::from(0.0, INFINITY), &mut rec) {
            return 0.5 * (rec.normal + Color::from(1.0, 1.0, 1.0));
        }
        let unit_direction = vec3::unit_vector(ray.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - a) * Color::from(1.0, 1.0, 1.0)
            + a * Color::from(0.5, 0.7, 1.0);
    }
}
