use std::ops::*;
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3
{
    #[inline]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    #[inline]
    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    #[inline]
    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    #[inline]
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn normalized(&self) -> Self {
        (1.0 / self.length()) * self.clone()
    }

    #[inline]
    pub fn normalize(&mut self) {
        let scalar = 1.0 / self.length();
        self.scale(scalar);
    }

    #[inline]
    pub fn scale(&mut self, scalar: f64) {
        *self *= scalar;
    }

    #[inline]
    pub fn scaled(&self, scalar: f64) -> Self {
        *self * scalar
    }

    #[inline]
    pub fn translate(&mut self, translation: Self) {
        *self += translation;
    }

    #[inline]
    pub fn translated(&self, translation: Self) -> Self {
        *self + translation
    }

    #[inline]
    pub fn rotate(&mut self, angles_radians: Self) {
        *self = self.rotated(angles_radians);
    }

    #[inline]
    pub fn rotated(&self, angles_radians: Self) -> Self {
        let sin_y = angles_radians.x.sin(); // roll
        let cos_y = angles_radians.x.cos();
        let sin_b = angles_radians.y.sin(); // pitch
        let cos_b = angles_radians.y.cos();
        let sin_a = angles_radians.z.sin(); // yaw
        let cos_a = angles_radians.z.cos();
        let mut x = cos_a * cos_b * self.x;
        x += (cos_a * sin_b * sin_y - sin_a * cos_y) * self.y;
        x += (cos_a * sin_b * cos_y + sin_a * sin_y) * self.z;
        let mut y = sin_a * cos_b * self.x;
        y += (sin_a * sin_b * sin_y + cos_a * cos_y) * self.y;
        y += (sin_a * sin_b * cos_y - cos_a * sin_y) * self.z;
        let mut z = - sin_b * self.x;
        z += cos_b * sin_y * self.y;
        z += cos_b * cos_y * self.z;
        Vec3::new(x, y, z)
    }

    #[inline]
    pub fn scale_non_uniform(&mut self, scale_vec: Self) {
        self.x *= scale_vec.x;
        self.y *= scale_vec.y;
        self.z *= scale_vec.z;
    }

    #[inline]
    pub fn scaled_non_uniform(&self, scale_vec: Self) -> Self {
        Self {
            x: self.x * scale_vec.x,
            y: self.y * scale_vec.y,
            z: self.z * scale_vec.z,
        }
    }

    #[inline]
    pub fn reflect(&self, normal: Vec3) -> Vec3 {
        *self - 2.0 * self.dot(normal) * normal
    }

    #[inline]
    pub fn refract(&self, normal: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min((-*self).dot(normal), 1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * normal);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * normal;
        r_out_perp + r_out_parallel
    }
}


impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn mul(self, _rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    #[inline]
    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self * _rhs.x,
            y: self * _rhs.y,
            z: self * _rhs.z,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    #[inline]
    fn div(self, _rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl ops::MulAssign<f64> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, scalar: f64) {
        *self = *self * scalar;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    #[inline]
    fn div_assign(&mut self, scalar: f64) {
        *self = *self / scalar;
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    #[inline]
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32
}

impl Vec2i {
    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add<Vec2i> for Vec2i {
    type Output = Vec2i;
    #[inline]
    fn add(self, _rhs: Vec2i) -> Vec2i {
        Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl From<Vec3> for Vec2i {
    #[inline]
    fn from(vec: Vec3) -> Self {
        Self {
            x: vec.x as i32,
            y: vec.y as i32,
        }
    }
}