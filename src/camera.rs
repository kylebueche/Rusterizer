use std::ptr::fn_addr_eq;
use crate::image::*;
use crate::vector::*;
use crate::raytracing::*;
use crate::color::*;
use std::mem::swap;



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

    pub fn trace_rays(&mut self) {

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


        println!("pixel00: {:?}, deltau: {:?}, deltav: {:?}", pixel_00_center, pixel_delta_u, pixel_delta_v);
        let sphere = Sphere::new(Vec3::new(0.0, 0.8, 5.0), 1.0);
        for x in 0..self.viewport.width {
            for y in 0..self.viewport.height {
                let fx = x as f64;
                let fy = y as f64;
                let pixel_point = pixel_00_center + pixel_delta_u * fx + pixel_delta_v * fy;
                let ray = Ray::new(self.position, pixel_point - self.position);
                if (x < 10 && y < 10) || (x > 244 && y > 244) {
                    println!("x: {}, y: {}, point: {:?}, direction: {:?}", x, y, pixel_point, ray.direction);

                }
                let mut interval = Interval::new(0.001, f64::INFINITY);
                //println!("Interval Before: {:?}", interval);
                let (hit_sphere, hit_point, hit_normal) = ray_sphere_intersection(ray, sphere, &mut interval);
                //println!("Interval After: {:?}\n", interval);
                if hit_sphere {
                    self.viewport.over(x, y, Col3f64::new(hit_normal.x, hit_normal.y, hit_normal.z), 1.0);
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