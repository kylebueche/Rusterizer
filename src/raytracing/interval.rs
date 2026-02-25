#[derive(Copy, Clone, Debug)]
pub struct Interval {
    pub lower_bound: f64,
    pub upper_bound: f64,
}

impl Interval {
    pub const fn new(lower_bound: f64, upper_bound: f64) -> Self {
        Self { lower_bound, upper_bound }
    }

    pub fn from_intervals(a: Interval, b: Interval) -> Self {
        Self {
            lower_bound: if a.lower_bound <= b.lower_bound { a.lower_bound } else { b.lower_bound },
            upper_bound: if a.upper_bound >= b.upper_bound { a.upper_bound } else { b.upper_bound },
        }
    }

    pub fn contains(&self, t: f64) -> bool {
        t >= self.lower_bound && t <= self.upper_bound
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Interval::new(self.lower_bound - padding, self.upper_bound + padding)
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
    pub const EMPTY: Interval = Interval::new(f64::INFINITY, f64::NEG_INFINITY);
    #[expect(unused)]
    pub const UNIVERSE: Interval = Interval::new(f64::NEG_INFINITY, f64::INFINITY);
}