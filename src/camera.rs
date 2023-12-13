use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::ray::Ray;
use crate::utils::degrees_to_radians;
use crate::vec3::{self};
use crate::vec3::{Color, Point3, Vec3};
use crate::{hittable, utils};
use std::sync::{Arc, Mutex};

use std::f64::INFINITY;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: f64, //

    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,

    pub defocus_angle: f64,
    pub focus_dist: f64,

    image_height: i32,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    center: Point3,

    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,

    // Camera frame basis Vectors
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            samples_per_pixel: 100,
            max_depth: 10,
            vfov: 90.0,
            image_height: 225,

            defocus_angle: 0.0,
            focus_dist: 10.0,

            lookfrom: Point3::from(0.0, 0.0, -1.0),
            lookat: Point3::new(),
            vup: Point3::from(0.0, 1.0, 0.0),

            pixel00_loc: Point3::new(),
            pixel_delta_u: Vec3::new(),
            pixel_delta_v: Vec3::new(),
            center: Point3::new(),

            u: Vec3::new(),
            v: Vec3::new(),
            w: Vec3::new(),

            defocus_disk_u: Vec3::new(),
            defocus_disk_v: Vec3::new(),
        }
    }

    pub fn initialize(&mut self) {
        self.image_height =
            (self.image_width as f64 / self.aspect_ratio) as i32;

        if self.image_height < 1 {
            self.image_height = 1;
        }
        self.center = self.lookfrom;

        // Determine the viewport dimensions
        // let focal_length = (self.lookfrom - self.lookat).length();
        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height
            * (self.image_width as f64 / self.image_height as f64);

        // Calculate the u, v, w unit basis vectors for the camera coordinate frame
        self.w = vec3::unit_vector(self.lookfrom - self.lookat);
        self.u = vec3::unit_vector(vec3::cross(&self.w, &self.vup));
        self.v = vec3::cross(&self.w, &self.u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u = viewport_width * self.u;
        let viewport_v = -viewport_height * -self.v;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate the location of the upper left pixel.

        let viewport_upper_left = self.center
            - (self.focus_dist * self.w)
            - viewport_u / 2.0
            - viewport_v / 2.0;

        self.pixel00_loc = viewport_upper_left
            + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = self.focus_dist
            * (utils::degrees_to_radians(self.defocus_angle / 2.0)).tan();

        self.defocus_disk_u = defocus_radius * self.u;
        self.defocus_disk_v = defocus_radius * self.v;
    }

    pub fn render(&mut self, world: &HittableList) {
        self.initialize();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            log::info!(r"Scanlines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += self.ray_color(&ray, self.max_depth, &world);
                }
                vec3::write_color(
                    std::io::stdout(),
                    &pixel_color,
                    self.samples_per_pixel,
                );
            }
        }

        log::info!("Done");
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Get a randomly-sampled camera ray for the pixel at location i, j, origination from
        // the camera defocus disk
        let pixel_center = self.pixel00_loc
            + (i * self.pixel_delta_u)
            + (j * self.pixel_delta_v);

        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        return Ray::from(ray_origin, ray_direction);
    }

    fn defocus_disk_sample(&self) -> Point3 {
        //Returns a random point in the camera defocus disk
        let p = vec3::random_in_unit_disk();
        return self.center
            + (p.x() * self.defocus_disk_u)
            + (p.y() * self.defocus_disk_v);
    }

    fn pixel_sample_square(&self) -> Vec3 {
        //Returns a random point in thesquare surrounding a pixel at the origin
        let px = -0.5 + crate::utils::random_double();
        let py = -0.5 + crate::utils::random_double();
        return (px * self.pixel_delta_u) + (py * self.pixel_delta_v);
    }
    fn ray_color(
        &self,
        ray: &Ray,
        depth: i32,
        world: &HittableList,
    ) -> vec3::Color {
        let mut rec = HitRecord::new();

        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::from(0.0, 0.0, 0.0);
        }

        if world.hit(ray, hittable::Interval::from(0.001, INFINITY), &mut rec) {
            // Temporarily take the material out and replace with None
            if let Some(material) = rec.mat.take() {
                let mut scattered = Ray::new();
                let mut attenuation = Color::new();

                if material.scatter(ray, &rec, &mut attenuation, &mut scattered)
                {
                    // Put it back
                    rec.mat = Some(material);
                    return attenuation
                        * self.ray_color(&scattered, depth - 1, world);
                }

                // put it back
                rec.mat = Some(material);
                return Color::new();
            }
            return Color::new();
        }
        let unit_direction = vec3::unit_vector(ray.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - a) * Color::from(1.0, 1.0, 1.0)
            + a * Color::from(0.5, 0.7, 1.0);
    }
}
