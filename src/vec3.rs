use std::ops;

use crate::hittable::Interval;
#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub e: [f64; 3],
}
pub type Color = Vec3;
pub type Point3 = Vec3;

impl Vec3 {
    /// Initializes a vector with all components set to zero.
    /// ```     
    /// assert_eq!(Vec3::new(), Vec3 { e: [0.0, 0.0, 0.0] });
    /// ```
    pub fn new() -> Self {
        Self { e: [0.0, 0.0, 0.0] }
    }
    pub fn from(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }
    pub fn length_squared(&self) -> f64 {
        return self.e[0] * self.e[0]
            + self.e[1] * self.e[1]
            + self.e[2] * self.e[2];
    }

    pub fn length(&self) -> f64 {
        return self.length_squared().sqrt();
    }

    fn random() -> Self {
        return Self::from(
            crate::utils::random_double(),
            crate::utils::random_double(),
            crate::utils::random_double(),
        );
    }

    fn random_in(min: f64, max: f64) -> Vec3 {
        return Self::from(
            crate::utils::random_double_range(min, max),
            crate::utils::random_double_range(min, max),
            crate::utils::random_double_range(min, max),
        );
    }
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_in(-1.0, 1.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

fn random_unit_vector() -> Vec3 {
    return unit_vector(random_in_unit_sphere());
}

pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
    let on_unit_sphere = random_in_unit_sphere();

    if dot(&on_unit_sphere, normal) > 0.0 {
        return on_unit_sphere;
    } else {
        return -on_unit_sphere;
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        return Vec3::from(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs);
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        return Vec3::from(self * rhs.e[0], self * rhs.e[1], self * rhs.e[2]);
    }
}

impl ops::Mul<i64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self::Output {
        return Vec3::from(
            self.e[0] * rhs as f64,
            self.e[1] * rhs as f64,
            self.e[2] * rhs as f64,
        );
    }
}

impl ops::Mul<Vec3> for i32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        return Vec3::from(
            self as f64 * rhs.e[0],
            self as f64 * rhs.e[1],
            self as f64 * rhs.e[2],
        );
    }
}

impl ops::Mul<i32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        return Vec3::from(
            self.e[0] * rhs as f64,
            self.e[1] * rhs as f64,
            self.e[2] * rhs as f64,
        );
    }
}

impl ops::Mul<Vec3> for i64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        return Vec3::from(
            self as f64 * rhs.e[0],
            self as f64 * rhs.e[1],
            self as f64 * rhs.e[2],
        );
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self::Output {
        return Vec3::from(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        );
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}
impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Self::Output {
        return Vec3::from(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        );
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.e[0] -= rhs.e[0];
        self.e[1] -= rhs.e[1];
        self.e[2] -= rhs.e[2];
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        return Vec3::from(-self.e[0], -self.e[1], -self.e[2]);
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        return self * (1.0 / rhs);
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    return u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2];
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    return Vec3::from(
        u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0],
    );
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    let k = v.length();

    return v / k;
}

pub fn write_color<W: std::io::Write>(
    mut out: W,
    pixel_color: &Color,
    samples_per_pixel: i32,
) {
    // Divide the color by the number of samples
    let scale = 1.0 / samples_per_pixel as f64;

    let r = pixel_color.x() * scale;
    let g = pixel_color.y() * scale;
    let b = pixel_color.z() * scale;

    let mut intensity = Interval::from(0.0, 1.0);

    write!(
        &mut out,
        "{} {} {} \n",
        (256.0 * intensity.clamp(r)) as i32,
        (256.0 * intensity.clamp(g)) as i32,
        (256.0 * intensity.clamp(b)) as i32
    )
    .expect("Failed to write to out in print_color");
}
