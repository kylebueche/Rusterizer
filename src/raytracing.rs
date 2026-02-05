use crate::vector::*;

#[derive(Copy, Clone, Debug)]
pub struct Interval {
    pub lower_bound: f64,
    pub upper_bound: f64,
}

impl Interval {
    pub fn new(lower_bound: f64, upper_bound: f64) -> Self {
        Self { lower_bound, upper_bound }
    }

    pub fn contains(&self, t: f64) -> bool {
        t >= self.lower_bound && t <= self.upper_bound
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}

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
    pub fn normal_at(&self, point: Vec3) -> Vec3 {
        (point - self.position).normalized()
    }
}

pub fn ray_sphere_intersection(ray: Ray, sphere: Sphere, interval: &mut Interval) -> (bool, Vec3, Vec3)
{
    let oc = sphere.position - ray.origin;
    let a = ray.direction.dot(ray.direction);
    let b = -2.0 * oc.dot(ray.direction);//sphere.position.dot(ray.direction);
    let c = oc.dot(oc) - sphere.radius_squared; //ray.origin.dot(ray.origin)
        /*
        + 2.0 * (ray.direction.dot(ray.origin) - sphere.position.dot(ray.origin))
        + sphere.position.dot(sphere.position)
        - sphere.radius_squared;
        */
    let (hit_sphere, t0, t1) = quadratic_formula(a, b, c);
    let mut t = 0.0;
    if (interval.contains(t0)) {
        interval.upper_bound = t0;
        t = t0;
    }
    if (interval.contains(t1)) {
        interval.upper_bound = t1;
        t = t1;
    }
    let hit_point = ray.at(t);
    let hit_normal = sphere.normal_at(hit_point);
    return (hit_sphere, hit_point, hit_normal);
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