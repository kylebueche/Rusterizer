use crate::raytracing::hittable::{HitRecord, Hittable};
use crate::raytracing::implicits::quadratic_formula;
use crate::raytracing::interval::*;
use crate::raytracing::ray::*;
use crate::solid::Solid;
use crate::vector::Vec3;
use crate::Arc;
use crate::raytracing::material::Material;
use std::sync::Mutex;


pub struct Sphere {
    pub position: Vec3,
    pub radius: f64,
    pub radius_squared: f64,
    pub mat: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(position: Vec3, radius: f64, mat: Arc<dyn Material>) -> Sphere {
        Sphere {
            position: position,
            radius: radius,
            radius_squared: radius * radius,
            mat: mat,
        }
    }

    // Parameter "point" is always expected to be on the surface.
    fn normal_at(&self, point: Vec3) -> Vec3 {
        (1.0 / self.radius) * (point - self.position)
    }
}

// optimization: can use modified quadratic formula with h substitution
impl Hittable for Sphere {
    fn first_hit_on_interval(&self, ray: Ray, interval: &mut Interval, hit_record: &mut HitRecord) -> bool
    {
        let oc = self.position - ray.origin;
        let a = ray.direction.dot(ray.direction);
        let b = -2.0 * oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius_squared;
        let (solution_exists, t0, t1) = quadratic_formula(a, b, c);
        let hit_sphere = solution_exists && (interval.contains(t0) || interval.contains(t1));
        if !hit_sphere {
            return hit_sphere;
        }
        if interval.contains(t0) {
            interval.upper_bound = t0;
            hit_record.t = t0;
            hit_record.point = ray.at(t0);
            let outward_normal = self.normal_at(hit_record.point);
            hit_record.set_face_normal(ray, outward_normal);
            hit_record.mat = Some(self.mat.clone());
        }
        if interval.contains(t1) {
            interval.upper_bound = t1;
            hit_record.t = t1;
            hit_record.point = ray.at(t1);
            let outward_normal = self.normal_at(hit_record.point);
            hit_record.set_face_normal(ray, outward_normal);
            hit_record.mat = Some(self.mat.clone());
        }

        hit_sphere
    }
}

impl Solid for Sphere {
    fn is_point_inside(&self, point: Vec3) -> bool {
        (point - self.position).length_squared() < self.radius_squared
    }
}