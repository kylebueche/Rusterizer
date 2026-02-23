use std::ops::*;
use std::ops;
use num_traits::{ Float, NumAssign, NumCast };

trait Scalar : Float + NumAssign + NumCast {}

#[derive(Copy, Clone, Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T: Scalar> Vec3<T>
{
    #[inline]
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
    #[inline]
    pub fn dot(&self, other: Self) -> T {
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
    pub fn length_squared(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    #[inline]
    pub fn length(&self) -> T {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn normalized(&self) -> Self {
        (T::one() / self.length()) * self.clone()
    }

    #[inline]
    pub fn normalize(&mut self) {
        let scalar = T::one() / self.length();
        self.scale(scalar);
    }

    #[inline]
    pub fn scale(&mut self, scalar: T) {
        *self *= scalar;
    }

    #[inline]
    pub fn scaled(&self, scalar: T) -> Self {
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
    pub fn reflect(&self, normal: Self) -> Self {
        *self - T::from(2.0) * self.dot(normal) * normal
    }

    #[inline]
    pub fn refract(&self, normal: Self, etai_over_etat: T) -> Self {
        let cos_theta = T::min((-*self).dot(normal), 1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * normal);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * normal;
        r_out_perp + r_out_parallel
    }
}


impl<T: Scalar> ops::Add<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    #[inline]
    fn add(self, _rhs: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

impl<T: Scalar> ops::Sub<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    #[inline]
    fn sub(self, _rhs: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z,
        }
    }
}

impl<T: Scalar> ops::Mul<T> for Vec3<T> {
    type Output = Vec3<T>;
    #[inline]
    fn mul(self, _rhs: T) -> Vec3<T> {
        Vec3 {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

impl<T: Scalar> ops::Mul<Vec3<T>> for T {
    type Output = Vec3<T>;
    #[inline]
    fn mul(self, _rhs: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self * _rhs.x,
            y: self * _rhs.y,
            z: self * _rhs.z,
        }
    }
}

impl<T: Scalar> ops::Div<T> for Vec3<T> {
    type Output = Vec3<T>;
    #[inline]
    fn div(self, _rhs: T) -> Vec3<T> {
        Vec3 {
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
    }
}

impl<T: Scalar> ops::AddAssign<Vec3<T>> for Vec3<T> {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl<T: Scalar> ops::SubAssign<Vec3<T>> for Vec3<T> {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl<T: Scalar> ops::MulAssign<T> for Vec3<T> {
    #[inline]
    fn mul_assign(&mut self, scalar: T) {
        *self = *self * scalar;
    }
}

impl<T: Scalar> ops::DivAssign<T> for Vec3<T> {
    #[inline]
    fn div_assign(&mut self, scalar: T) {
        *self = *self / scalar;
    }
}

impl<T: Scalar> ops::Neg for Vec3<T> {
    type Output = Vec3<T>;
    #[inline]
    fn neg(self) -> Vec3<T> {
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

impl<T: Scalar> From<Vec3<T>> for Vec2i {
    #[inline]
    fn from(vec: Vec3<T>) -> Self {
        Self {
            x: vec.x.to_i32().unwrap(),
            y: vec.y.to_i32().unwrap(),
        }
    }
}