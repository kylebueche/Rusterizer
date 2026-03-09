use std::sync::Arc;
use crate::color::*;
use crate::vector::*;
use stb_image::stb_image::{stbi_loadf, stbi_image_free, stbi_set_flip_vertically_on_load};
use std::fs;
use std::ffi::CString;
use std::path::Path;

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

struct Texture2D {
    bytes_per_pixel: usize,
    fdata: *mut f64,
    width: usize,
    height: usize,
    channels: usize,
}

impl Texture2D {
    pub fn load<P: AsRef<Path>>(path: P) -> Option<Self> {
        let c_path = CString::new(path.as_ref().to_str()?).ok()?;
        let mut width = 0;
        let mut height = 0;
        let mut channels = 0;
        let bytes_per_pixel = 3;

        unsafe {
            stbi_set_flip_vertically_on_load(1);

            let data = stbi_loadf(c_path.as_ptr(), &mut width, &mut height, &mut channels, 3);
            if data.is_null() {
                None
            } else {
                Some(Texture2D {
                    fdata: data as *mut f64,
                    width: width as usize,
                    height: height as usize,
                    channels: channels as usize,
                    bytes_per_pixel
                })
            }
        }
    }
}