use std::ops::*;
use std::ops;

use crate::scalar::Scalar;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3
{
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn normalized(&self) -> Self {
        (1.0 / self.length()) * self.clone()
    }
    pub fn normalize(&mut self) {
        let scalar = 1.0 / self.length();
        self.scale(scalar);
    }
    pub fn scale(&mut self, scalar: f64) {
        *self *= scalar;
    }

    pub fn scaled(&self, scalar: f64) -> Self {
        *self * scalar
    }

    pub fn translate(&mut self, translation: Self) {
        *self += translation;
    }
    pub fn translated(&self, translation: Self) -> Self {
        *self + translation
    }
    pub fn rotate(&mut self, angles_radians: Self) {
        *self = self.rotated(angles_radians);
    }
    pub fn rotated(&self, angles_radians: Self) -> Self {
        let sin_y = angles_radians.x.sin(); // roll
        let cos_y = angles_radians.x.cos();
        let sin_B = angles_radians.y.sin(); // pitch
        let cos_B = angles_radians.y.cos();
        let sin_a = angles_radians.z.sin(); // yaw
        let cos_a = angles_radians.z.cos();
        let mut x = cos_a * cos_B * self.x;
        x += (cos_a * sin_B * sin_y - sin_a * cos_y) * self.y;
        x += (cos_a * sin_B * cos_y + sin_a * sin_y) * self.z;
        let mut y = sin_a * cos_B * self.x;
        y += (sin_a * sin_B * sin_y + cos_a * cos_y) * self.y;
        y += (sin_a * sin_B * cos_y - cos_a * sin_y) * self.z;
        let mut z = - sin_B * self.x;
        z += cos_B * sin_y * self.y;
        z += cos_B * cos_y * self.z;
        Vec3::new(x, y, z)
    }
    pub fn scale_non_uniform(&mut self, scale_vec: Self) {
        self.x *= scale_vec.x;
        self.y *= scale_vec.y;
        self.z *= scale_vec.z;
    }

    pub fn scaled_non_uniform(&self, scale_vec: Self) -> Self {
        Self {
            x: self.x * scale_vec.x,
            y: self.y * scale_vec.y,
            z: self.z * scale_vec.z,
        }
    }

}


impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
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
    fn div(self, _rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, scalar: f64) {
        *self = *self * scalar;
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, scalar: f64) {
        *self = *self / scalar;
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
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
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add<Vec2i> for Vec2i {
    type Output = Vec2i;
    fn add(self, _rhs: Vec2i) -> Vec2i {
        Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl From<Vec3> for Vec2i {
    fn from(vec: Vec3) -> Self {
        Self {
            x: vec.x as i32,
            y: vec.y as i32,
        }
    }
}