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
use progress_bar::*;

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
    pixel_samples_scale: f64,
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
            pixel_samples_scale: 0.1,
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
            pixel_samples_scale: 0.1,
        }
    }

    pub fn initialize(&mut self) {
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
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

    pub fn render(&mut self, scene_objects: Arc<dyn Hittable>) {
        //let mut rng = fastrand::Rng::new();
        init_progress_bar(self.viewport.data.len());
        self.initialize();
        //let scene_objects = Arc::new(scene_objects);
        let pixel_samples_scale = 1.0 / (self.samples_per_pixel as f64);
        for y in 0..self.viewport.height {
            for x in 0..self.viewport.width {
                inc_progress_bar();
                let mut pixel_color = Col3f64::new(0.0, 0.0, 0.0);
                for sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    //pixel_color += self.ray_color(ray, scene_objects, self.max_depth);
                }
                pixel_color *= pixel_samples_scale;
                *self.viewport.index_2d_mut(x, y) = pixel_color;
            }
            let percent = 100.0 * (y as f64) / (self.viewport.height as f64);
            let percent_int = percent as i32;
            let percent_fract = (percent.fract() * 100.0) as i32;
            print!("\rPercent complete: {}.{}%", percent_int, percent_fract);
        }
        finalize_progress_bar();
    }
    pub fn render_threaded(&mut self, scene_objects: &impl Hittable) {
        self.initialize();

        let mut img = Image::with_dimensions(self.viewport.width, self.viewport.height);
        let num_chunks = img.height;
        let chunk_size = img.data.len() / num_chunks;
        init_progress_bar(num_chunks);
        let mut total_samples = Arc::new(Mutex::new(0.0));
        let mut peak_samples = Arc::new(Mutex::new(0.0));
        img.data.par_chunks_mut(chunk_size).enumerate().for_each(|(chunk_number, row)|{
            let mut total_chunk_samples = 0.0;
            let mut peak = 0.0;
            for i in 0..row.len() {
                let index =  chunk_number * chunk_size + i;
                //self.pixel_kernel(index, img.width, scene_objects, &mut row[i]);
                let curr_samples= self.convergent_kernel(0.80, index, img.width, scene_objects, &mut row[i]);
                total_chunk_samples += curr_samples;
                peak = f64::max(peak, curr_samples);

                //let y = index / img.width;
                //let x = index % img.width;
                //let mut pixel_color = Col3f64::new(0.0, 0.0, 0.0);
                //let mut old_color = pixel_color;
                //for sample in 0..self.samples_per_pixel {
                //    let ray = self.get_ray(x, y);
                //    pixel_color += self.ray_color(ray, scene_objects, self.max_depth);
                //}
                //pixel_color *= self.pixel_samples_scale;
                //row[i] = linear_to_srgb(pixel_color);
            }
            inc_progress_bar();
            let mut samps = total_samples.lock().unwrap();
            *samps += total_chunk_samples;
            let mut peaks = peak_samples.lock().unwrap();
            *peaks += peak;

        });
        finalize_progress_bar();

        let samps = total_samples.lock().unwrap();
        let peak = peak_samples.lock().unwrap();
        println!("\nDone. {} samples and {} samples per pixel", *samps, *samps / (img.data.len() as f64));
        println!("{} was the peak number of samples in a pixel", *peak);
        self.viewport = img;
    }

    pub fn render_threaded_alternate(&mut self, scene_objects: &impl Hittable) {
        self.initialize();
        let pixel_samples_scale = 1.0 / (self.samples_per_pixel as f64);

        let mut img = Image::with_dimensions(self.viewport.width, self.viewport.height);
        img.data.par_iter_mut().enumerate().for_each(|(index, pixel)| {
            self.pixel_kernel(index, img.width, scene_objects, pixel);
            let y = index / img.width;
            let x = index % img.width;
            let mut pixel_color = Col3f64::new(0.0, 0.0, 0.0);
            for sample in 0..self.samples_per_pixel {
                let ray = self.get_ray(x, y);
                pixel_color += self.ray_color(ray, scene_objects, self.max_depth);
            }
            pixel_color *= pixel_samples_scale;
            *pixel = linear_to_gamma(pixel_color);

        });

        self.viewport = img;
    }

    #[inline]
    fn pixel_kernel(&self, index: usize, width: usize, scene_objects: &impl Hittable, pixel: &mut Col3f64) {
        let y = index / width;
        let x = index % width;
        let mut pixel_color = Col3f64::new(0.0, 0.0, 0.0);
        for sample in 0..self.samples_per_pixel {
            let ray = self.get_ray(x, y);
            pixel_color += self.ray_color(ray, scene_objects, self.max_depth);
        }
        pixel_color *= self.pixel_samples_scale;
        *pixel = linear_to_srgb(pixel_color);
    }

    /*
     * A convergent pixel kernel.
     * This kernel samples infinitely until a certain proportion of the most recent samples
     * have made no change to the pixel value.
     * Each pixel will be sampled at least self.samples_per_pixel times
     * Then, if the ratio is 0.2, samples will be added until the most recent 20% of samples
     * change the pixel by less than 1.0/255.0 in each channel.
     * Due to the nature of Monte Carlo integration, convergence is guaranteed for
     * any convergence ratio in [0.0, 1.0). Convergence is impossible for [1.0, inf), or (-inf, 0.0)
     */
    #[inline]
    fn convergent_kernel(&self, convergence_ratio: f64, index: usize, width: usize, scene_objects: &impl Hittable, pixel: &mut Col3f64) -> f64 {
        let y = index / width;
        let x = index % width;
        let mut pixel_color = Col3f64::new(0.0, 0.0, 0.0);
        let mut old_color = pixel_color;
        let mut num_samples = 0;
        let mut num_samples_unchanged = 0;
        loop {
            let ray = self.get_ray(x, y);
            let sample = self.ray_color(ray, scene_objects, self.max_depth);
            pixel_color += sample;
            num_samples += 1;
            let new_color = pixel_color / num_samples as f64;
            let diff = new_color - old_color;
            if diff.x.abs() < 1.0 / 255.0 && diff.y.abs() < 1.0 / 255.0 && diff.z.abs() < 1.0 / 255.0 {
                num_samples_unchanged += 1;
            } else {
                num_samples_unchanged = 0;
                old_color = pixel_color / num_samples as f64;
            }
            if (num_samples_unchanged as f64) / (num_samples as f64) > convergence_ratio && num_samples > self.samples_per_pixel {
                break;
            }
        }
        pixel_color /= num_samples as f64;
        *pixel = linear_to_srgb(pixel_color);
        num_samples as f64
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
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.position
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_range(0.0..1.0);
        Ray::with_time(ray_origin, ray_direction, ray_time)
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

fn linear_to_srgb_float(linear_component: f64) -> f64 {
    if linear_component <= 0.0031308 {
        12.92 * linear_component
    } else {
        1.055 * linear_component.powf(1.0 / 2.4) - 0.055
    }
}

fn linear_to_srgb(linear_color: Col3f64) -> Col3f64 {
    Col3f64 {
        x: linear_to_srgb_float(linear_color.x),
        y: linear_to_srgb_float(linear_color.y),
        z: linear_to_srgb_float(linear_color.z),
    }
}