use crate::hittable;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3;
use crate::vec3::{Point3, Vec3};
use std::rc::Rc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Option<Rc<dyn Material>>,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            center: Point3::new(),
            radius: 0.0,
            mat: None,
        }
    }
    pub fn from(center: Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat: Some(mat),
        }
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        ray: &Ray,
        ray_t: hittable::Interval,
        rec: &mut HitRecord,
    ) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();

        let half_b = vec3::dot(&oc, &ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        };

        let sqrtd = discriminant.sqrt();

        //Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;

            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);

        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        if let Some(mat) = &self.mat {
            rec.mat = Some(mat.clone());
        }

        return true;
    }
}
