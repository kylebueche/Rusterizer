use crate::vector::Vec3;
use crate::raytracing::ray::Ray;
use crate::raytracing::aabb::AABB;
use std::sync::Arc;
use crate::raytracing::hittable::{HitRecord, Hittable};
use crate::raytracing::interval::Interval;
use crate::raytracing::material::*;
use crate::raytracing::implicits::plane::*;


pub struct Quad {
    q: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Arc<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    d: f64,
}

impl Quad {
    pub fn new(q: Vec3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
        let bbox_diagonal1 = AABB::from_corners(q, q + u + v);
        let bbox_diagonal2 = AABB::from_corners(q + u, q + v);
        let bbox = AABB::from_aabbs(bbox_diagonal1, bbox_diagonal2);
        let n = u.cross(v);
        let normal = n.normalized();
        let d = normal.dot(q);
        let w = n / n.dot(n);
        Self { q, u, v, w, mat, bbox, normal, d }
    }

    fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool{
        let unit_interval = Interval::new(0.0, 1.0);
        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            false
        } else {
            rec.u = a;
            rec.v = b;
            true
        }
    }
}

unsafe impl Sync for Quad {}
unsafe impl Send for Quad {}

impl Hittable for Quad {
    fn first_hit_on_interval(&self, ray: Ray, interval: &mut Interval, hit_record: &mut HitRecord) -> bool {
        let denom = self.normal.dot(ray.direction);
        if denom.abs() < 1e-8 {
            false
        } else {
            let t = (self.d - self.normal.dot(ray.origin)) / denom;
            if !interval.contains(t) {
                false
            } else {
                let intersection = ray.at(t);
                let planar_hitpt_vector = intersection - self.q;
                let alpha = self.w.dot(planar_hitpt_vector.cross(self.v));
                let beta = self.w.dot(self.u.cross(planar_hitpt_vector));
                if !Self::is_interior(alpha, beta, hit_record) {
                    false
                } else {
                    interval.upper_bound = t;
                    hit_record.t = t;
                    hit_record.point = intersection;
                    hit_record.mat = Some(self.mat.clone());
                    hit_record.set_face_normal(ray, self.normal);
                    true
                }
            }
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}