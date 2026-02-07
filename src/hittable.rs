use crate::vector::*;
use crate::raytracing::*;

pub trait Hittable {
    fn first_hit_on_interval(&self, ray: Ray, interval: &mut Interval) -> (bool, f64);
    fn normal_at(&self, point: Vec3) -> Vec3;
    fn is_point_inside(&self, point: Vec3) -> bool;
}