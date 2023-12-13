use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::utils;
use crate::vec3;
use crate::vec3::{Color, Vec3};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn from(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::from(rec.p, scatter_direction);
        *attenuation = self.albedo;
        return true;
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}
impl Metal {
    pub fn from(albedo: Vec3, fuzz: f64) -> Self {
        if fuzz < 1.0 {
            Self { albedo, fuzz }
        } else {
            Self { albedo, fuzz: 1.0 }
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected =
            vec3::reflect(&vec3::unit_vector(r_in.direction()), &rec.normal);
        *scattered = Ray::from(rec.p, reflected);
        *attenuation = self.albedo;
        return vec3::dot(&scattered.direction(), &rec.normal) > 0.0;
    }
}

pub struct Dielectric {
    ir: f64, // Index of Refraction
}

impl Dielectric {
    pub fn from(ir: f64) -> Self {
        Self { ir }
    }

    fn reflectance(&self, cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);

        r0 = r0 * r0;

        return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
    }
}
impl Material for Dielectric {
    fn scatter(
        &self,
        ray: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = vec3::unit_vector(ray.direction());

        let cos_theta = vec3::dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract
            || self.reflectance(cos_theta, refraction_ratio)
                > utils::random_double()
        {
            direction = vec3::reflect(&unit_direction, &rec.normal);
        } else {
            direction =
                vec3::refract(&unit_direction, &rec.normal, refraction_ratio);
        }

        *attenuation = Color::from(1.0, 1.0, 1.0);
        *scattered = Ray::from(rec.p, direction);

        return true;
    }
}
