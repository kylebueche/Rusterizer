use crate::image::*;
use crate::vector::*;
use crate::color::*;
use crate::raytracing::hittable::*;
use crate::raytracing::interval::*;
use crate::raytracing::ray::*;
use crate::random::*;
use crate::threadbatcher::ThreadPool;
use std::sync::{Mutex, Arc};
use rayon::prelude::*;

pub enum RayTracingMethod {
    Unoptimized,
    Threaded,
    SIMD,
    ThreadedPlusSIMD,
}

pub struct Camera {
    pub viewport: Image,
    pub aspect_ratio: f64,
    pub position: Vec3,
    pub front: Vec3,
    pub up: Vec3,
    pub samples_per_pixel: usize,
    pub max_depth: usize,
    pub field_of_view: f64,
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    // uninit
    right: Vec3,
    focal_length: f64,
    viewport_height: f64,
    viewport_width: f64,
    pixel00_top_left: Vec3,
    pixel00_center: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
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
            look_from: Vec3::new(0.0, 0.0, 0.0),
            look_at: Vec3::new(0.0, 0.0, -1.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            // uninit:
            right: Vec3::new(1.0, 0.0, 0.0),
            viewport_width: 1.0,
            viewport_height: 1.0,
            pixel00_top_left: Vec3::new(0.0, 0.0, 0.0),
            pixel00_center: Vec3::new(0.0, 0.0, 0.0),
            focal_length: 1.0,
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            u: Vec3::new(0.0, 0.0, 0.0),
            v: Vec3::new(0.0, 0.0, 0.0),
            w: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_u: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_v: Vec3::new(0.0, 0.0, 0.0),
            // unused:
            field_of_view: 90.0,
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
            look_from: Vec3::new(0.0, 0.0, 0.0),
            look_at: Vec3::new(0.0, 0.0, -1.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            // uninit:
            right: Vec3::new(1.0, 0.0, 0.0),
            viewport_width: 1.0,
            viewport_height: 1.0,
            pixel00_top_left: Vec3::new(0.0, 0.0, 0.0),
            pixel00_center: Vec3::new(0.0, 0.0, 0.0),
            focal_length: 1.0,
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            u: Vec3::new(0.0, 0.0, 0.0),
            v: Vec3::new(0.0, 0.0, 0.0),
            w: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_u: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_v: Vec3::new(0.0, 0.0, 0.0),
            // unused:
            field_of_view: 90.0,
        }
    }

    pub fn initialize(&mut self) {
        self.position = self.look_from;
        let theta = self.field_of_view.to_radians();
        let h = (theta / 2.0).tan();
        self.viewport_width = 2.0 * h * self.focus_dist;
        self.viewport_height = self.viewport_width / self.aspect_ratio;
        self.w = (self.look_from - self.look_at).normalized();
        self.u = (self.up.cross(self.w)).normalized();
        self.v = self.w.cross(self.u);
        let viewport_u = self.u * self.viewport_width;
        let viewport_v = -self.v * self.viewport_height;
        self.pixel_delta_u = (1.0 / self.viewport.width as f64) * viewport_u;
        self.pixel_delta_v = (1.0 / self.viewport.height as f64) * viewport_v;
        self.pixel00_top_left =
            self.position - self.focus_dist * self.w
                - viewport_u / 2.0
                - viewport_v / 2.0;
        self.pixel00_center = self.pixel00_top_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    pub fn render_threaded(&mut self, scene_objects: Arc<dyn Hittable>) {
        let logger_mutex = Mutex::new(0);
        self.initialize();
        let pixel_samples_scale = 1.0 / (self.samples_per_pixel as f64);
        let mut img = Image::with_dimensions(self.viewport.width, self.viewport.height);
        img.data.par_chunks_mut(img.width).enumerate().for_each(|(y, row)|{
            //let scene_objects = scene_objects.clone();
                for x in 0..img.width {
                    let mut pixel_color = Col3f64::new(0.0, 0.0, 0.0);
                    for sample in 0..self.samples_per_pixel {
                        let ray = self.get_ray(x, y);
                        pixel_color += self.ray_color(ray, &scene_objects, self.max_depth);
                    }
                    pixel_color *= pixel_samples_scale;
                    row[x] = linear_to_gamma(pixel_color);
                }
            // Progress logging
            let mut val = logger_mutex.lock().unwrap();
            *val = *val + 1;
            let percent = 100.0 * (*val as f64) / (3.0 * img.height as f64);
            let percent_int = percent as i32;
            let percent_fract = (percent.fract() * 100.0) as i32;
            print!("\rPercent complete: {}.{}%", percent_int, percent_fract);
            std::mem::drop(val);
            //}
        });
        self.viewport = img;

        // alternate render loop using tiles
        /*
        let chunk_width = 64;
        let chunk_height = 64;
                let viewport_width = self.viewport.width;
                let viewport_height = self.viewport.height;
                for chunk_x in (0..viewport_width).step_by(chunk_width) {
                    for chunk_y in (0..viewport_width).step_by(chunk_height) {
                        let percent = 100.0 * (chunk_x + chunk_width * chunk_y) as f64 / (self.viewport.height * self.viewport.width) as f64;
                        let percent_int = (percent) as i32;
                        let percent_fract = (percent.fract() * 100.0) as i32;
                        print!("\rPercent complete: {}.{}%", percent_int, percent_fract);

                        let start_x = chunk_x;
                        let start_y = chunk_y;
                        let end_x = (chunk_x + chunk_width).min(viewport_width);
                        let end_y = (chunk_y + chunk_height).min(viewport_height);
                        //thread_pool.execute(move || {
                            for x in start_x..end_x {
                                for y in start_y..end_y {
                                    let mut pixel_color = Col3f64::new(0.0, 0.0, 0.0);
                                    for sample in 0..self.samples_per_pixel {
                                        let ray = self.get_ray(x, y);
                                        pixel_color += self.ray_color(ray, scene_objects, self.max_depth);
                                    }
                                    pixel_color = pixel_color * pixel_samples_scale;
                                    *self.viewport.index_2d_mut(x, y) = linear_to_gamma(pixel_color);
                                }
                            }
                        //});
                    }
                }
                */
    }

    fn ray_color(&self, ray: Ray, scene_objects: &Arc<dyn Hittable>, depth: usize) -> Vec3{
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
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.position
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let point = random_in_unit_disk();
        self.position + point.x * self.defocus_disk_u + point.y * self.defocus_disk_v
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