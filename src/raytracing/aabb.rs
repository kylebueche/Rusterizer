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

    pub fn hit(&self, ray: Ray, ray_t: &mut Interval) -> bool {
        let mut ray_t = ray_t.clone();
        let ray_pos = ray.origin;
        let ray_dir = ray.direction;
        if !Self::axis_overlap_check(ray_pos.x, ray_dir.x, self.x, &mut ray_t) {
            return false
        } else if !Self::axis_overlap_check(ray_pos.y, ray_dir.y, self.y, &mut ray_t) {
            return false
        } else if !Self::axis_overlap_check(ray_pos.z, ray_dir.z, self.z, &mut ray_t) {
            return false
        }
        true
    }

    #[inline]
    fn axis_overlap_check(ray_pos: f64, ray_dir: f64, axis: Interval, ray_t: &mut Interval) -> bool{
        if ray_dir == 0.0 {
            if ray_pos < axis.lower_bound || ray_pos > axis.upper_bound {
                return false
            }
            return true
        }

        let adinv = 1.0 / ray_dir;
        let mut t0 = (axis.lower_bound - ray_pos) * adinv;
        let mut t1 = (axis.upper_bound - ray_pos) * adinv;
        //if t0 < t1 {
        //    if t0 > ray_t.lower_bound { ray_t.lower_bound = t0; }
        //    if t1 < ray_t.upper_bound { ray_t.upper_bound = t1; }
        //} else {
        //    if t1 > ray_t.lower_bound { ray_t.lower_bound = t1; }
        //    if t0 < ray_t.upper_bound { ray_t.upper_bound = t0; }
        //}

        let t_min_axis = t0.min(t1);
        let t_max_axis = t0.max(t1);
        ray_t.lower_bound = ray_t.lower_bound.max(t_min_axis);
        ray_t.upper_bound = ray_t.upper_bound.min(t_max_axis);



        //if ray_t.upper_bound <= ray_t.lower_bound {
        //    return false
        //}
        //true
        ray_t.upper_bound >= ray_t.lower_bound
    }
}