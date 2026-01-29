use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Col3u8 {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

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

#[derive(Copy, Clone, Debug)]
pub struct Col3f64 {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

impl Col3f64 {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

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
}

impl From<Col3u8> for Col3f64 {
    fn from(_from: Col3u8) -> Self {
        (1.0 / 255.0) * Self {
            r: _from.r as f64,
            g: _from.g as f64,
            b: _from.b as f64
        }
    }
}

impl From<Col3f64> for Col3u8 {
    fn from(_from: Col3f64) -> Self {
        let _from = 255.99 * _from;
        Self {
            r: _from.r as u8,
            g: _from.g as u8,
            b: _from.b as u8
        }
    }
}


impl ops::Add<Col3u8> for Col3u8 {
    type Output = Col3u8;
    fn add(self, _rhs: Col3u8) -> Col3u8 {
        Col3u8 {
            r: self.r + _rhs.r,
            g: self.g + _rhs.g,
            b: self.b + _rhs.b,
        }
    }
}


impl ops::Add<Col3f64> for Col3f64 {
    type Output = Col3f64;
    fn add(self, _rhs: Col3f64) -> Col3f64 {
        Col3f64 {
            r: self.r + _rhs.r,
            g: self.g + _rhs.g,
            b: self.b + _rhs.b
        }
    }
}

impl ops::Mul<Col3f64> for f64 {
    type Output = Col3f64;
    fn mul(self, _rhs: Col3f64) -> Col3f64 {
        Col3f64 {
            r: self * _rhs.r,
            g: self * _rhs.g,
            b: self * _rhs.b,
        }
    }
}