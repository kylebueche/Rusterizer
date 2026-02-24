use crate::raytracing::aabb::*;
use crate::raytracing::hittable::*;
use std::sync::Arc;

struct BVHNode {
    pub bbox: AABB,
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
}



impl Hittable for BVHNode {}