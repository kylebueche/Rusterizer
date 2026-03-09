use crate::vector::*;
use crate::raytracing::ray::*;
use crate::raytracing::interval::*;
use std::sync::Arc;
use std::rc::Rc;
use crate::raytracing::material::*;
use crate::raytracing::aabb::*;

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