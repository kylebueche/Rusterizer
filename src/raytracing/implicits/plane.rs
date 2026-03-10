use crate::raytracing::hittable::{HitRecord, Hittable};
use crate::raytracing::interval::Interval;
use crate::raytracing::ray::Ray;
use crate::raytracing::aabb::AABB;
use crate::vector::Vec3;

pub struct Plane {
    pub position: Vec3,
    pub normal: Vec3,
}

impl Plane {
    pub fn new(position: Vec3, normal: Vec3) -> Self {
        Self { position, normal }
    }
}

impl Hittable for Plane {
    fn first_hit_on_interval(&self, ray: Ray, interval: &mut Interval, hit_record: &mut HitRecord) -> bool {
        false
    }
    
    fn bounding_box(&self) -> AABB {
        AABB::UNIVERSE
    }
}