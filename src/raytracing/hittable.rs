use crate::vector::*;
use crate::raytracing::ray::*;
use crate::raytracing::interval::*;
use std::sync::Arc;
use std::rc::Rc;
use crate::color::Color;
use crate::random::random_range;
use crate::raytracing::material::*;
use crate::raytracing::aabb::*;
use crate::raytracing::texture::*;

#[derive(Clone)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat: Option<Arc<dyn Material>>,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 1.0, 0.0),
            t: 0.0,
            u: 0.0,
            v: 0.0,
            front_face: true,
            mat: Option::None,
        }
    }
    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}

pub trait Hittable: Send + Sync {
    fn first_hit_on_interval(&self, ray: Ray, interval: &mut Interval, hit_record: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> AABB;
}


// can't derive traits? weird, has to do with dyn
pub struct HittableList {
    pub hittables: Vec<Arc<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            hittables: vec![],
            bbox: AABB::EMPTY,
        }
    }

    pub fn add(&mut self, hittable: Arc<dyn Hittable>) {
        self.bbox = AABB::from_aabbs(self.bbox, hittable.bounding_box());
        self.hittables.push(hittable);
    }

    #[expect(unused)]
    pub fn clear(&mut self) {
        self.hittables.clear();
    }
}

// potential improvement:
// only calculate hit point and normals for minimum t value.
// hard to do because you lose track of the
impl Hittable for HittableList {
    fn first_hit_on_interval(&self, ray: Ray, interval: &mut Interval, hit_record: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        for hittable in &self.hittables {
            if hittable.first_hit_on_interval(ray, interval, hit_record) {
                hit_anything = true;
            }
        }
        hit_anything
    }
    fn bounding_box(&self) -> AABB {
        AABB::EMPTY
    }
}

pub struct HittableStaticList<T: Hittable> {
    hittables: Vec<T>,
    bbox: AABB,
}

impl<T: Hittable> HittableStaticList<T> {
    pub fn new() -> HittableStaticList<T> {
        HittableStaticList {
            hittables: vec![],
            bbox: AABB::EMPTY,
        }
    }
    
    pub fn add(&mut self, hittable: T) {
        self.bbox = AABB::from_aabbs(self.bbox, hittable.bounding_box());
        self.hittables.push(hittable);
    }
    
    pub fn clear(&mut self) {
        self.hittables.clear();
    }
}

impl<T: Hittable> Hittable for HittableStaticList<T> {
    fn first_hit_on_interval(&self, ray: Ray, interval: &mut Interval, hit_record: &mut HitRecord) -> bool {
        let mut hit_anything = false;for hittable in &self.hittables {
            if hittable.first_hit_on_interval(ray, interval, hit_record) {
                hit_anything = true;
            }
        }
        hit_anything
    }
    fn bounding_box(&self) -> AABB {
        AABB::EMPTY
    }
}

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: AABB,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Translate {
        let bbox = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn first_hit_on_interval(&self, ray: Ray, interval: &mut Interval, hit_record: &mut HitRecord) -> bool {
        let offset_r = Ray::with_time(ray.origin - self.offset, ray.direction, ray.time);
        if !self.object.first_hit_on_interval(offset_r, interval, hit_record) {
            return false;
        }
        hit_record.point += self.offset;
        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> RotateY {
        let radians = f64::to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Vec3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Vec3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let id = i as f64;
                    let jd = j as f64;
                    let kd = k as f64;
                    let x = id * bbox.x.upper_bound + (1.0 - id) * bbox.x.lower_bound;
                    let y = jd * bbox.y.upper_bound + (1.0 - jd) * bbox.y.lower_bound;
                    let z = kd * bbox.z.upper_bound + (1.0 - kd) * bbox.z.lower_bound;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    min.x = f64::min(min.x, tester.x);
                    max.x = f64::min(max.x, tester.x);
                    min.y = f64::min(min.y, tester.y);
                    max.y = f64::min(max.y, tester.y);
                    min.z = f64::min(min.z, tester.z);
                    max.z = f64::min(max.z, tester.z);
                }
            }
        }
        let bbox = AABB::from_corners(min, max);
        Self {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn first_hit_on_interval(&self, ray: Ray, interval: &mut Interval, hit_record: &mut HitRecord) -> bool {
        let origin = Vec3 {
            x: self.cos_theta * ray.origin.x - self.sin_theta * ray.origin.z,
            y: ray.origin.y,
            z: self.sin_theta * ray.origin.x + self.cos_theta * ray.origin.z,
        };

        let direction = Vec3 {
            x: self.cos_theta * ray.direction.x - self.sin_theta * ray.direction.z,
            y: ray.direction.y,
            z: self.sin_theta * ray.direction.x + self.cos_theta * ray.direction.z,
        };

        let rotated_ray = Ray::with_time(origin, direction, ray.time);

        if !self.object.first_hit_on_interval(rotated_ray, interval, hit_record) {
            return false;
        }

        hit_record.point = Vec3 {
            x: self.cos_theta * hit_record.point.x + self.sin_theta * hit_record.point.z,
            y: hit_record.point.y,
            z: -self.sin_theta * hit_record.point.x + self.cos_theta * hit_record.point.z,
        };

        hit_record.normal = Vec3 {
            x: self.cos_theta * hit_record.normal.x + self.sin_theta * hit_record.normal.z,
            y: hit_record.normal.y,
            z: -self.sin_theta * hit_record.normal.x + self.cos_theta * hit_record.normal.z,
        };

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, density: f64, albedo: Color) -> ConstantMedium {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::new(albedo))
        }
    }
    pub fn from_texture(boundary: Arc<dyn Hittable>, density: f64, tex: Arc<dyn Texture>) -> ConstantMedium {
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::from_texture(tex)),
        }
    }
}

unsafe impl Sync for ConstantMedium{}
unsafe impl Send for ConstantMedium{}

impl Hittable for ConstantMedium {
    fn first_hit_on_interval(&self, ray: Ray, interval: &mut Interval, hit_record: &mut HitRecord) -> bool {
        let mut rec1 = HitRecord::new();
        let mut rec2 = HitRecord::new();
        let mut interval1 = Interval::UNIVERSE;
        if !self.boundary.first_hit_on_interval(ray, &mut interval1, &mut rec1) {
            return false;
        }
        let mut interval2 = Interval::new(rec1.t + 0.0001, f64::INFINITY);

        if !self.boundary.first_hit_on_interval(ray, &mut interval2, &mut rec2) {
            return false;
        }

        if rec1.t < interval.lower_bound { rec1.t = interval.lower_bound };
        if rec2.t > interval.upper_bound { rec2.t = interval.upper_bound };

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = ray.direction.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * f64::ln(random_range(0.0..1.0));
        
        if hit_distance > distance_inside_boundary {
            return false;
        }
        
        hit_record.t = rec1.t + hit_distance / ray_length;
        hit_record.point = ray.at(hit_record.t);
        hit_record.normal = Vec3::new(1.0, 0.0, 0.0);
        hit_record.front_face = true;
        hit_record.mat = Some(self.phase_function.clone());
        
        true
    }
    
    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }
}