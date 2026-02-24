use crate::raytracing::aabb::*;
use crate::raytracing::hittable::*;
use crate::raytracing::interval::*;
use crate::raytracing::ray::*;
use std::sync::Arc;

struct BVHNode {
    pub bbox: AABB,
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
}

impl BVHNode {
    pub fn new(list: HittableList) -> Self {
        Self {

        }
    }

    pub fn
}

impl Hittable for BVHNode {
    fn first_hit_on_interval(&self, ray: Ray, interval: &mut Interval, hit_record: &mut HitRecord) -> bool {
        if !self.bbox.hit()
    }
}