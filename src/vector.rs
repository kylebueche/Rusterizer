use std::ops;
use crate::color::Col3f64;

#[derive(Copy)]
#[derive(Clone)]
pub struct Vec2f64 {
    pub x: f64,
    pub y: f64
}

pub fn dot(a: Vec2f64, b: Vec2f64) -> f64 {
    a.x * b.x + a.y * b.y
}

impl Vec2f64 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn normalized(&self) -> Vec2f64 {
        (1.0 / self.length()) * self.clone()
    }
}

impl ops::Add<Vec2f64> for Vec2f64 {
    type Output = Vec2f64;
    fn add(self, _rhs: Vec2f64) -> Vec2f64 {
        Vec2f64 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl ops::Sub<Vec2f64> for Vec2f64 {
    type Output = Vec2f64;
    fn sub(self, _rhs: Vec2f64) -> Vec2f64 {
        Vec2f64 {
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
        }
    }
}

impl ops::Mul<Vec2f64> for f64 {
    type Output = Vec2f64;
    fn mul(self, _rhs: Vec2f64) -> Vec2f64 {
        Vec2f64 {
            x: self * _rhs.x,
            y: self * _rhs.y,
        }
    }
}

#[derive(Copy)]
#[derive(Clone)]
pub struct Vec2f32 {
    pub x: f32,
    pub y: f32
}


impl Vec2f32 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}


#[derive(Copy)]
#[derive(Clone)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32
}

impl Vec2i {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}