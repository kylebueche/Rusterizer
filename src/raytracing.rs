use crate::vector::*;

#[derive(Copy, Clone, Debug)]
pub struct Interval {
    pub lower_bound: f64,
    pub upper_bound: f64,
}

impl Interval {
    pub const fn new(lower_bound: f64, upper_bound: f64) -> Self {
        Self { lower_bound, upper_bound }
    }

    pub fn contains(&self, t: f64) -> bool {
        t >= self.lower_bound && t <= self.upper_bound
    }

    pub fn size(&self) -> f64 {
        self.upper_bound - self.lower_bound
    }

    pub fn surrounds(&self, t: f64) -> bool {
        t > self.lower_bound && t < self.upper_bound
    }

    const empty: Interval = Interval::new(f64::INFINITY, f64::NEG_INFINITY);
    const universe: Interval = Interval::new(f64::NEG_INFINITY, f64::INFINITY);
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