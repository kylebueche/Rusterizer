use crate::vector::*;

#[derive(Copy, Clone, Debug)]
pub struct Interval {
    pub lower_bound: f64,
    pub upper_bound: f64,
}

impl Interval {
    pub fn new(lower_bound: f64, upper_bound: f64) -> Self {
        Self { lower_bound, upper_bound }
    }

    pub fn contains(&self, t: f64) -> bool {
        t >= self.lower_bound && t <= self.upper_bound
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}