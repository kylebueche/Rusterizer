use crate::raytracing::hittable::{HitRecord, Hittable};
use crate::raytracing::implicits::quadratic_formula;
use crate::raytracing::interval::*;
use crate::raytracing::ray::*;
use crate::solid::Solid;
use crate::vector::Vec3;
use crate::Arc;
use crate::raytracing::material::Material;
use std::sync::Mutex;

#[derive(Clone)]
pub struct Sphere {
    pub position: Ray,
    pub radius: f64,
    pub radius_squared: f64,
    pub mat: Arc<dyn Material>,
}

unsafe impl Sync for Sphere {}
unsafe impl Send for Sphere {}

impl Sphere {
    pub fn new(position: Vec3, radius: f64, mat: Arc<dyn Material>) -> Sphere {
        Sphere {
            position: Ray::new(position, Vec3::new(0.0, 0.0, 0.0)),
            radius: radius,
            radius_squared: radius * radius,
            mat: mat,
        }
    }

    pub fn new_moving(position1: Vec3, position2: Vec3, radius: f64, mat: Arc<dyn Material>) -> Sphere {
        Sphere {
            position: Ray::new(position1, position2 - position1),
            radius: radius,
            radius_squared: radius * radius,
            mat: mat,
        }
    }

    // Parameter "point" is always expected to be on the surface.
    // This is incorrect
    fn normal_at(&self, point: Vec3) -> Vec3 {
        (1.0 / self.radius) * (point - self.position.origin)
    }
}


// optimization: can use modified quadratic formula with h substitution
impl Hittable for Sphere {
    fn first_hit_on_interval(&self, ray: Ray, interval: &mut Interval, hit_record: &mut HitRecord) -> bool
    {
        let sphere_position = self.position.at(ray.time);
        let oc = sphere_position - ray.origin;
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
            let outward_normal = (hit_record.point - sphere_position) / self.radius; //self.normal_at(hit_record.point);
            hit_record.set_face_normal(ray, outward_normal);
            hit_record.mat = Some(self.mat.clone());
        }
        if interval.contains(t1) {
            interval.upper_bound = t1;
            hit_record.t = t1;
            hit_record.point = ray.at(t1);
            let outward_normal = (hit_record.point - sphere_position) / self.radius;
            hit_record.set_face_normal(ray, outward_normal);
            hit_record.mat = Some(self.mat.clone());
        }

        hit_sphere
    }
}

impl Solid for Sphere {
    // not correct for moving spheres.
    fn is_point_inside(&self, point: Vec3) -> bool {
        (point - self.position.origin).length_squared() < self.radius_squared
    }
}