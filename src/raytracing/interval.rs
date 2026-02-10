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

    #[expect(unused)]
    pub fn size(&self) -> f64 {
        self.upper_bound - self.lower_bound
    }

    #[expect(unused)]
    pub fn surrounds(&self, t: f64) -> bool {
        t > self.lower_bound && t < self.upper_bound
    }

    #[expect(unused)]
    const EMPTY: Interval = Interval::new(f64::INFINITY, f64::NEG_INFINITY);
    #[expect(unused)]
    const UNIVERSE: Interval = Interval::new(f64::NEG_INFINITY, f64::INFINITY);
}