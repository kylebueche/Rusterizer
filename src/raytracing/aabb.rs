use crate::raytracing::hittable::{HitRecord, Hittable};
use crate::raytracing::interval::Interval;
use crate::raytracing::ray::Ray;
use crate::vector::*;

#[derive(Copy, Clone)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub const EMPTY: Self = Self {
        x: Interval::EMPTY,
        y: Interval::EMPTY,
        z: Interval::EMPTY,
    };

    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_corners(a: Vec3, b: Vec3) -> Self {
        Self {
            x: if a.x <= b.x { Interval::new(a.x, b.x) } else { Interval::new(b.x, a.x) },
            y: if a.y <= b.y { Interval::new(a.y, b.y) } else { Interval::new(b.y, a.y) },
            z: if a.z <= b.z { Interval::new(a.z, b.z) } else { Interval::new(b.z, a.z) },
        }
    }

    pub fn from_aabbs(a: AABB, b: AABB) -> Self {
        Self {
            x: Interval::from_intervals(a.x, b.x),
            y: Interval::from_intervals(a.y, b.y),
            z: Interval::from_intervals(a.z, b.z),
        }
    }

    pub fn axis_interval(&self, n: i32) -> &Interval {
        if n == 1 {
            &self.y
        } else if n == 2 {
            &self.z
        } else {
            &self.x
        }
    }

    fn hit(&self, ray: Ray, ray_t: &mut Interval) -> bool {
        let ray_pos = ray.origin;
        let ray_dir = ray.direction;
        if !Self::axis_overlap_check(ray_pos.x, ray_dir.x, self.x, ray_t) {
            return false
        } else if !Self::axis_overlap_check(ray_pos.y, ray_dir.y, self.y, ray_t) {
            return false
        } else if !Self::axis_overlap_check(ray_pos.z, ray_dir.z, self.z, ray_t) {
            return false
        }
        true
    }

    #[inline]
    fn axis_overlap_check(ray_pos: f64, ray_dir: f64, axis: Interval, ray_t: &mut Interval) -> bool{
        let adinv = 1.0 / ray_dir;
        let mut t0 = (axis.lower_bound - ray_pos) * adinv;
        let mut t1 = (axis.upper_bound - ray_pos) * adinv;
        if t0 > t1 {
            std::mem::swap(&mut t0, &mut t1);
        }
        if t0 > ray_t.lower_bound { ray_t.lower_bound = t0; }
        if t1 < ray_t.upper_bound { ray_t.upper_bound = t1; }

        if ray_t.upper_bound < ray_t.lower_bound {
            return false
        }
        true
    }
}
/*
impl Hittable for aabb {
    fn first_hit_on_interval(&self, ray: Ray, interval: &mut Interval, hit_record: &mut HitRecord) -> bool {
        let ray_origin = ray.origin;
        let ray_dir = ray.direction;
        // x axis
        let axis = self.x;
        let adinv = 1.0 / ray_dir.x;

        let mut t0 = (axis.lower_bound - ray.origin.x) * adinv;
        let mut t1 = (axis.upper_bound - ray.origin.x) * adinv;
        if t0 < t1 {
            if t0 > ray_t
        }

        true
    }
}*/