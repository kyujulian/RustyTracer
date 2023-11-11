use crate::vec3::{Point3, Vec3};
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new() -> Self {
        Self {
            orig: Point3::new(),
            dir: Vec3::new(),
        }
    }
    pub fn from(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
    }
    pub fn origin(&self) -> Point3 {
        self.orig
    }
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    pub fn at(&mut self, t: f64) -> Vec3 {
        return self.orig + t * self.dir;
    }
}
