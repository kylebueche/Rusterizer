use std::sync::Arc;
use crate::color::*;
use crate::vector::*;
use stb_image::stb_image::{stbi_loadf, stbi_load, stbi_image_free, stbi_set_flip_vertically_on_load};
use std::fs;
use std::ffi::CString;
use std::path::Path;
use std::ptr::null_mut;
use crate::image::Image;
use crate::raytracing::interval::Interval;
use crate::random;

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Color;
}

pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        SolidColor { albedo }
    }
    
    pub fn new_rgb(red: f64, green: f64, blue: f64) -> Self {
        SolidColor::new(Color::new(red, green, blue))
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Color {
        self.albedo
    }
}

pub struct CheckerTexture {
    pub inv_scale: f64,
    pub even: Arc<dyn Texture>,
    pub odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Color, odd: Color) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Arc::new(SolidColor::new(even)),
            odd: Arc::new(SolidColor::new(odd)),
        }
    }

    pub fn from_textures(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: even,
            odd: odd,
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Color {
        let x_int = (self.inv_scale * p.x).floor() as i32;
        let y_int = (self.inv_scale * p.y).floor() as i32;
        let z_int = (self.inv_scale * p.z).floor() as i32;

        let is_even = (x_int + y_int + z_int) % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}

#[derive(Clone)]
struct RTWImage {
    bytes_per_pixel: usize,
    fdata: *mut f32, // Linear floating poing pixel data
    bdata: *mut u8,  // Linear 8-bit pixel data
    width: usize,
    height: usize,
    bytes_per_scanline: usize,
}

impl RTWImage {
    pub fn new() -> Self {
        Self {
            bytes_per_pixel: 3,
            fdata: null_mut(),
            bdata: null_mut(),
            width: 0,
            height: 0,
            bytes_per_scanline: 0
        }
    }
    pub fn from_path<P: AsRef<Path>>(path: P) -> Option<Self> {
        let path: String = path.as_ref().to_str()?.to_string();
        let dir = "textures".to_owned();
        unsafe {
            let mut tex = RTWImage::new();
            if tex.load(dir.clone() + "/" + &path)
                || tex.load(path.clone())
                || tex.load("../".to_owned() + &dir + "/" + &path)
                || tex.load("../../".to_owned() + &dir + "/" + &path)
                || tex.load("../../../".to_owned() + &dir + "/" + &path)
            {
                println!("here2");
                Some(tex)
            } else {
                None
            }
        }
    }
    pub fn load(&mut self, path: String) -> bool {
        let c_path = CString::new(path).unwrap();
        let mut width = 0;
        let mut height = 0;
        let mut channels = 0;
        let mut n = self.bytes_per_pixel as i32; // Dummy output param: original components per pixel

        unsafe {
            stbi_set_flip_vertically_on_load(1);

            self.bdata = stbi_load(c_path.as_ptr(), &mut width, &mut height, &mut n, self.bytes_per_pixel as i32);
            if self.bdata.is_null() {
                false
            } else {
                self.width = width as usize;
                self.height = height as usize;
                self.bytes_per_scanline = self.width * self.bytes_per_pixel;
                println!("here");
                //self.convert_to_bytes();
                true
            }
        }
    }

    pub fn pixel_data(&self, x: usize, y: usize) -> &[u8 ] {
        static MAGENTA: [u8;3] = [255, 0, 255];
        if self.bdata.is_null() {
            &MAGENTA
        } else {
            let x = Self::clamp(x, 0, self.width);
            let y = Self::clamp(y, 0, self.height);
            let idx = y * self.bytes_per_scanline + x * self.bytes_per_pixel;
            unsafe {
                &std::slice::from_raw_parts(self.bdata as *const u8, self.bytes_per_pixel * self.height * self.width)[idx..(idx + self.bytes_per_pixel)]
            }
        }
    }

    fn clamp(x: usize, low: usize, high: usize) -> usize {
        if x < low {
            low
        } else if x < high {
            x
        } else {
            high - 1
        }
    }

    pub fn width(&self) -> usize {
        if self.bdata.is_null() {
            0
        } else {
            self.width
        }
    }

    pub fn height(&self) -> usize {
        if self.bdata.is_null() {
            0
        } else {
            self.height
        }
    }

    pub fn float_to_byte(value: f32) -> u8 {
        f32::max(0.0, f32::min(255.99 * value, 255.99)) as u8
    }
/*
    pub fn convert_to_bytes(&mut self) {
        let total_bytes = self.width * self.height * self.bytes_per_pixel;
        unsafe {
            //self.bdata = std::alloc::alloc(std::alloc::Layout::new::<u8>());
            let bdata = std::slice::from_raw_parts_mut(self.bdata, total_bytes);
            let fdata = std::slice::from_raw_parts(self.fdata, total_bytes);
            for i in 0..total_bytes {
                bdata[i] = Self::float_to_byte(fdata[i]);
            }
        }
    }*/
}

impl Drop for RTWImage {
    fn drop(&mut self) {
        unsafe {
            if !self.bdata.is_null() {
                stbi_image_free(self.bdata as *mut _);
                //std::alloc::dealloc(self.bdata, std::alloc::Layout::new::<u8>());
            }
            //stbi_image_free(self.fdata as *mut _);
        }
    }
}

pub struct ImageTexture {
    image: RTWImage,
}

impl ImageTexture {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            image: RTWImage::from_path(path).unwrap(),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Vec3) -> Color {
        if self.image.height <= 0 || self.image.width <= 0 {
            Color::cyan()
        } else {
            let u = Interval::new(0.0, 1.0).clamp(u);
            let v = Interval::new(0.0, 1.0).clamp(v);

            let i = (u * self.image.width() as f64) as usize;
            let j = (v * self.image.height() as f64) as usize;

            //println!("u: {}, v: {}", self.image.width(), self.image.height());
            let pixel = self.image.pixel_data(i, j);
            let color_scale = 1.0 / 255.0;
            color_scale * Color::new(pixel[0] as f64, pixel[1] as f64, pixel[2] as f64)
        }
    }
}

struct Perlin {
    rand_vec: [Vec3; Perlin::POINT_COUNT],
    perm_x: [i32; Perlin::POINT_COUNT],
    perm_y: [i32; Perlin::POINT_COUNT],
    perm_z: [i32; Perlin::POINT_COUNT],
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new() -> Self {
        let mut perlin = Perlin {
            rand_vec: [Vec3::new(0.0, 0.0, 0.0); Perlin::POINT_COUNT],
            perm_x: [0; Perlin::POINT_COUNT],
            perm_y: [0; Perlin::POINT_COUNT],
            perm_z: [0; Perlin::POINT_COUNT],
        };
        for i in 0..Perlin::POINT_COUNT {
            perlin.rand_vec[i] = Vec3::random_range(-1.0..1.0);
        }

        Perlin::perlin_generate_perm(&mut perlin.perm_x);
        Perlin::perlin_generate_perm(&mut perlin.perm_y);
        Perlin::perlin_generate_perm(&mut perlin.perm_z);
        perlin
    }

    pub fn turb(&self, p: Vec3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;
        for i in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum.abs()
    }

    pub fn noise(&self, p: Vec3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.rand_vec[
                            (self.perm_x[((i + di as i32) & 255i32) as usize] ^
                            self.perm_y[((j + dj as i32) & 255i32) as usize] ^
                            self.perm_z[((k + dk as i32) & 255i32) as usize]) as usize];
                }
            }
        }
        Self::perlin_interp(c, u, v, w)
    }

    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                           * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                           * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                           * c[i][j][k].dot(weight_v);
                }
            }
        }
        accum
    }

    fn perlin_generate_perm(p: &mut [i32; Perlin::POINT_COUNT]) {
        for i in 0..Perlin::POINT_COUNT {
            p[i] = i as i32;
        }
        Perlin::permute(p, Perlin::POINT_COUNT);
    }

    fn permute(p: &mut [i32; Perlin::POINT_COUNT], n: usize) {
        for i in (1..(n - 1)).rev() {
            let target = random::random_int(0..(i as i32));
            let tmp = p[i];
            p[i] = p[target as usize];
            p[target as usize] = tmp;
        }
    }
}

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Color {
        Color::new(0.5, 0.5, 0.5) * (1.0 + f64::sin(self.scale * p.z + 10.0 * self.noise.turb(p, 7)))
    }
}