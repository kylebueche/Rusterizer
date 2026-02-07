use crate::vector::*;
use crate::hittable::*;
use crate::raytracing::{Interval, Ray};
use crate::solid::*;

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    pub position: Vec3,
    pub radius: f64,
    pub radius_squared: f64,
}

impl Sphere {
    pub fn new(position: Vec3, radius: f64) -> Sphere {
        Sphere {
            position: position,
            radius: radius,
            radius_squared: radius * radius,
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
        let (hit_sphere, t0, t1) = quadratic_formula(a, b, c);
        let mut t = 0.0;
        if (interval.contains(t0)) {
            interval.upper_bound = t0;
            t = t0;
            hit_record.t = t;
            hit_record.point = ray.at(t);
            let outward_normal = self.normal_at(hit_record.point);
            hit_record.set_face_normal(ray, outward_normal);
        }
        if (interval.contains(t1)) {
            interval.upper_bound = t1;
            t = t1;
            hit_record.t = t;
            hit_record.point = ray.at(t);
            let outward_normal = self.normal_at(hit_record.point);
            hit_record.set_face_normal(ray, outward_normal);
        }

        hit_sphere
    }
}

impl Solid for Sphere {
    fn is_point_inside(&self, point: Vec3) -> bool {
        (point - self.position).length_squared() < self.radius_squared
    }
}

fn quadratic_formula(a: f64, b: f64, c: f64) -> (bool, f64, f64) {
    let denominator = 2.0 * a;
    let inside_sqrt = b * b - 4.0 * a * c;
    let solution_exists = inside_sqrt >= 0.0 && denominator != 0.0;
    let root = inside_sqrt.sqrt();
    let t0 = (-b - root) / denominator;
    let t1 = (-b + root) / denominator;
    (solution_exists, t0, t1)
}