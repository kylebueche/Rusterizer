use crate::raytracing::aabb::*;
use crate::raytracing::hittable::*;
use crate::raytracing::interval::*;
use crate::raytracing::ray::*;
use crate::image::Image;
use crate::raytracing::camera::Camera;
use std::sync::Arc;
use crate::random::*;

pub struct BVHNode {
    pub bbox: AABB,
    pub left: Arc<dyn Hittable>,
    pub right: Arc<dyn Hittable>,
}

impl BVHNode {
    pub fn new(list: &mut HittableList) -> Self {
        let len = list.hittables.len();
        Self::new_from_indices(&mut list.hittables, 0, len)
    }

    pub fn new_from_indices(objects: &mut Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let mut bbox = AABB::EMPTY;
        for i in start..end {
            bbox = AABB::from_aabbs(bbox, objects[i].bounding_box());
        }
        let axis = bbox.longest_axis();// random_int(0..2);
        let comparator = if axis == 0 {
            Self::box_x_compare
        } else if axis == 1 {
            Self::box_y_compare
        } else {
            Self::box_z_compare
        };

        let span = end - start;
        let (left, right) = if span == 1 {
            (objects[start].clone(), objects[start].clone())
        } else if span == 2 {
            (objects[start].clone(), objects[start + 1].clone())
        } else {
            objects[start..end].sort_by(comparator);

            let mid = start + span / 2;
            let left: Arc<dyn Hittable> = Arc::new(BVHNode::new_from_indices(objects, start, mid));
            let right: Arc<dyn Hittable> = Arc::new(BVHNode::new_from_indices(objects, mid, end));
            (left, right)
        };

        Self {
            left: left,
            right: right,
            bbox: bbox,
        }
    }

    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: i32) -> std::cmp::Ordering {
        let a_axis_interval = *a.bounding_box().axis_interval(axis_index);
        let b_axis_interval = *b.bounding_box().axis_interval(axis_index);
        if a_axis_interval.lower_bound < b_axis_interval.lower_bound {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }

    fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 0)
    }
    fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 1)
    }
    fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> std::cmp::Ordering {
        Self::box_compare(a, b, 2)
    }


    //pub fn draw (&self, camera: &mut Camera) {
    //    image.
    //}
}

impl Hittable for BVHNode {
    fn first_hit_on_interval(&self, ray: Ray, interval: &mut Interval, hit_record: &mut HitRecord) -> bool {
        if !self.bbox.hit(ray, interval) {
            return false;
        }

        let mut interval = interval.clone();
        let hit_left = self.left.first_hit_on_interval(ray, &mut interval, hit_record);
        let mut right_interval = Interval::new(interval.lower_bound, if hit_left { hit_record.t } else { interval.upper_bound });
        let hit_right = self.right.first_hit_on_interval(ray, &mut right_interval, hit_record);
        hit_left || hit_right
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}