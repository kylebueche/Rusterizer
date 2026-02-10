use std::ops;
use crate::vector::*;

pub type Col3f64 = Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Col3u8 {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

/*
impl Col3u8 {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn white() -> Self {
        Self::new(255, 255, 255)
    }
    pub fn black() -> Self {
        Self::new(0, 0, 0)
    }
    pub fn red() -> Self {
        Self::new(255, 0, 0)
    }
    pub fn green() -> Self {
        Self::new(0, 255, 0)
    }
    pub fn blue() -> Self {
        Self::new(0, 0, 255)
    }
}
*/


impl Col3f64 {
    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }
    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
    pub fn red() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }
    pub fn green() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }
    pub fn blue() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }
    pub fn magenta() -> Self {
        Self::new(1.0, 0.0, 1.0)
    }
    pub fn yellow() -> Self {
        Self::new(1.0, 1.0, 0.0)
    }
    pub fn cyan() -> Self {
        Self::new(0.0, 1.0, 1.0)
    }
    #[inline]
    pub fn random() -> Self {
        Self {
            x: rand::random_range(0.0..1.0),
            y: rand::random_range(0.0..1.0),
            z: rand::random_range(0.0..1.0),
        }
    }
    #[inline]
    pub fn random_range(range: std::ops::Range<f64>) -> Self {
        Self {
            x: rand::random_range(range.clone()),
            y: rand::random_range(range.clone()),
            z: rand::random_range(range.clone()),
        }
    }
}

impl From<Col3u8> for Col3f64 {
    fn from(_from: Col3u8) -> Col3f64 {
        (1.0 / 255.0) * Col3f64 {
            x: _from.r as f64,
            y: _from.g as f64,
            z: _from.b as f64
        }
    }
}

impl From<Col3f64> for Col3u8 {
    fn from(_from: Col3f64) -> Col3u8 {
        let _from: Col3f64 = 255.99 * _from;
        Col3u8 {
            r: _from.x as u8,
            g: _from.y as u8,
            b: _from.z as u8,
        }
    }
}


impl ops::Mul<Col3f64> for Col3f64 {
    type Output = Col3f64;
    fn mul(self, _rhs: Col3f64) -> Col3f64 {
        Col3f64 {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
        }
    }
}