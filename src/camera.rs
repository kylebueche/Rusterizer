use crate::image::*;
use crate::vector::*;
use crate::raytracing::*;
use crate::color::*;
use crate::implicits::*;
use crate::hittable::*;
use std::sync::Arc;

pub struct Camera {
    pub viewport: Image,
    pub aspect_ratio: f64,
    pub position: Vec3,
    pub front: Vec3,
    pub up: Vec3,
    pub field_of_view: f64,
}

impl Camera {
    pub fn new(image_width: usize, image_height: usize) -> Self {
        Self {
            viewport: Image::with_dimensions(image_width, image_height),
            aspect_ratio: image_width as f64 / image_height as f64,
            position: Vec3::new(0.0, 0.0, 0.0),
            front: Vec3::new(0.0, 0.0, 1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            field_of_view: 45.0,
        }
    }

    pub fn render(&mut self, scene_objects: &impl Hittable) {

        // values to compute
        let right: Vec3 = self.front.cross(self.up);
        let half_fov = self.field_of_view / 2.0;
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * self.aspect_ratio;
        let viewport_u = right * viewport_width;
        let viewport_v = -self.up * viewport_height;
        let pixel_delta_u = (1.0 / self.viewport.width as f64) * viewport_u;
        let pixel_delta_v = (1.0 / self.viewport.height as f64) * viewport_v;
        let viewport_upper_left =
            self.position + self.front * focal_length
            - viewport_u * 0.5 - viewport_v * 0.5;
        let pixel_00_center = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let mut hit_record: HitRecord = HitRecord::new();
        for x in 0..self.viewport.width {
            for y in 0..self.viewport.height {
                let fx = x as f64;
                let fy = y as f64;
                let pixel_point = pixel_00_center + pixel_delta_u * fx + pixel_delta_v * fy;
                let ray = Ray::new(self.position, pixel_point - self.position);

                let mut interval = Interval::new(f64::EPSILON, f64::INFINITY);
                let hit_sphere = scene_objects.first_hit_on_interval(ray, &mut interval, &mut hit_record);

                if hit_sphere {
                    self.viewport.over(x, y, Col3f64::new(hit_record.normal.x/2.0 + 0.5, -hit_record.normal.y/2.0 + 0.5, -hit_record.normal.z/2.0 + 0.5), 1.0);
                } else {
                    let blue = Vec3::new(0.0, 0.0, 1.0);
                    let white = Vec3::new(1.0, 1.0, 1.0);
                    let lerp = (1.0 - ray.direction.y) * white + ray.direction.y * blue;
                    self.viewport.over(x, y, Col3f64::new(lerp.x, lerp.y, lerp.z), 1.0);
                }
            }
        }
    }
}