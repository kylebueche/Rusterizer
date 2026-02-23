use crate::raytracing::hittable::{HitRecord, Hittable};
use crate::raytracing::interval::Interval;
use crate::raytracing::ray::Ray;
use crate::vector::*;

struct aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl aabb {
    pub fn empty() -> Self {
        Self {
            x: Interval::EMPTY,
            y: Interval::EMPTY,
            z: Interval::EMPTY,
        }
    }
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

    pub fn axis_interval(&self, n: i32) -> &Interval {
        if n == 1 {
            &self.y
        } else if n == 2 {
            &self.z
        } else {
            &self.x
        }
    }

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
}

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
}