use crate::image::*;
use crate::vector::*;
use crate::color::*;
use crate::raytracing::hittable::*;
use crate::raytracing::interval::*;
use crate::raytracing::ray::*;
use crate::random::*;

pub struct Camera {
    pub viewport: Image,
    pub aspect_ratio: f64,
    pub position: Vec3,
    pub front: Vec3,
    pub up: Vec3,
    pub samples_per_pixel: usize,
    pub max_depth: usize,
    // uninit
    right: Vec3,
    focal_length: f64,
    viewport_height: f64,
    viewport_width: f64,
    pixel00_top_left: Vec3,
    pixel00_center: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    #[expect(unused)]
    pub field_of_view: f64,
}

impl Camera {
    #[expect(unused)]
    pub fn new(image_width: usize, image_height: usize) -> Self {
        Self {
            viewport: Image::with_dimensions(image_width, image_height),
            aspect_ratio: image_width as f64 / image_height as f64,
            position: Vec3::new(0.0, 0.0, 0.0),
            front: Vec3::new(0.0, 0.0, 1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            samples_per_pixel: 10,
            max_depth: 10,
            // uninit:
            right: Vec3::new(1.0, 0.0, 0.0),
            viewport_width: 1.0,
            viewport_height: 1.0,
            pixel00_top_left: Vec3::new(0.0, 0.0, 0.0),
            pixel00_center: Vec3::new(0.0, 0.0, 0.0),
            focal_length: 1.0,
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            // unused:
            field_of_view: 45.0,
        }
    }

    pub fn from_aspect_ratio(image_width: usize, aspect_ratio: f64) -> Self {
        let image_height = (image_width as f64) / aspect_ratio;
        let image_height = if image_height < 1.0 { 1usize } else { image_height as usize };
        Self {
            viewport: Image::with_dimensions(image_width, image_height),
            aspect_ratio: aspect_ratio,
            position: Vec3::new(0.0, 0.0, 0.0),
            front: Vec3::new(0.0, 0.0, 1.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            samples_per_pixel: 10,
            max_depth: 10,
            // uninit:
            right: Vec3::new(1.0, 0.0, 0.0),
            viewport_width: 1.0,
            viewport_height: 1.0,
            pixel00_top_left: Vec3::new(0.0, 0.0, 0.0),
            pixel00_center: Vec3::new(0.0, 0.0, 0.0),
            focal_length: 1.0,
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            // unused:
            field_of_view: 45.0,
        }
    }

    pub fn render(&mut self, scene_objects: &impl Hittable) {

        // values to compute
        self.right = self.front.cross(self.up);
        self.focal_length = 1.0;
        self.viewport_height = 2.0;
        self.viewport_width = self.viewport_height * self.aspect_ratio;
        let viewport_u = self.right * self.viewport_width;
        let viewport_v = -self.up * self.viewport_height;
        self.pixel_delta_u = (1.0 / self.viewport.width as f64) * viewport_u;
        self.pixel_delta_v = (1.0 / self.viewport.height as f64) * viewport_v;
        self.pixel00_top_left =
            self.position + self.front * self.focal_length
            - viewport_u * 0.5 - viewport_v * 0.5;
        self.pixel00_center = self.pixel00_top_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
        let pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        // render loop
        for x in 0..self.viewport.width {
            let fx = x as f64;
            let percent = 100.0 * fx / ((self.viewport.width - 1) as f64);
            let percent_int = percent as i32;
            let percent_fract = (percent.fract() * 100.0) as i32;
            print!("\rPercent complete: {}.{}%", percent_int, percent_fract);
            for y in 0..self.viewport.height {
                let mut pixel_color = Col3f64::new(0.0, 0.0, 0.0);
                let fy = y as f64;
                for sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    pixel_color += self.ray_color(ray, scene_objects, self.max_depth);
                }
                pixel_color *= pixel_samples_scale;
                *self.viewport.index_2d_mut(x, y) = linear_to_gamma(pixel_color);
            }
        }
    }

    fn ray_color(&self, ray: Ray, scene_objects: &impl Hittable, depth: usize) -> Vec3{
        if depth <= 0 {
            return Col3f64::new(0.0, 0.0, 0.0);
        }
        let mut hit_record = HitRecord::new();
        let mut interval = Interval::new(1.0e-8, f64::INFINITY);
        if scene_objects.first_hit_on_interval(ray, &mut interval, &mut hit_record) {
            let mut scattered: Ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
            let mut attenuation: Col3f64 = Col3f64::new(0.0, 0.0, 0.0);
            let mat = hit_record.mat.clone();
            if mat.unwrap().scatter(ray, &hit_record, &mut attenuation, &mut scattered) {
                return attenuation * self.ray_color(scattered, scene_objects, depth - 1);
            }
            return Col3f64::new(0.0, 0.0, 0.0);
        }
        let unit_direction = ray.direction.normalized();
        let a = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0);
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_center
        + (i as f64 + offset.x) * self.pixel_delta_u
        + (j as f64 + offset.y) * self.pixel_delta_v;
        let ray_origin = self.position;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }
}

fn linear_to_gamma_float(linear_component: f64) -> f64 {
    // technically the standard: ~x^0.45
    // linear_component.powf(1.0 / 2.2)
    // fast approximation: x^0.5
    linear_component.sqrt()
}

fn linear_to_gamma(linear_color: Col3f64) -> Col3f64 {
    Col3f64 {
        x: linear_to_gamma_float(linear_color.x),
        y: linear_to_gamma_float(linear_color.y),
        z: linear_to_gamma_float(linear_color.z),
    }
}