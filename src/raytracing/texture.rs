use crate::color::*;
use crate::vector::*;

trait Texture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Color;
}

struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        SolidColor { albedo }
    }
    
    pub fn new_rgb(red: f64, green: f64, blue: f64) -> Self {
        SolidColor::new(Vec3::new(red, green, blue))
    }
}

impl Texture for SolidColor {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Color {
        self.albedo
    }
}