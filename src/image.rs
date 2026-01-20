use std::fs;
use std::ops::{Index, IndexMut};
use crate::color::{Col3f64, Col3u8};

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Col3f64>,
}

impl Image {
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            data: Vec::new(),
        }
    }
    pub fn with_dimensions(width: usize, height: usize) -> Self {
        Self {
            width: width,
            height: height,
            data: vec![Col3f64::white(); 3 * width * height]
        }
    }

    pub fn index_2d(&self, x: usize, y: usize) -> &Col3f64 {
        &self.data[self.width * y + x]
    }

    pub fn index_2d_mut(&mut self, x: usize, y: usize) -> &mut Col3f64 {
        &mut self.data[self.width * y + x]
    }

    pub fn write_to_file(&self, filename: &str) {
        let mut ppm = String::new();
        let width = self.width;
        let height = self.height;
        ppm.push_str(format!("P3\n{width} {height}\n255\n").as_str());
        for y in (0..height) {
            for x in (0..width) {
                let i = (y * width + x);
                let col = Col3u8::from(self.index_2d(x, y).clone());
                ppm.push_str(format!("{} {} {}\n", col.r, col.g, col.b).as_str());
            }
        }

        let file_path = String::from(filename);
        fs::write(file_path, ppm).expect("File unable to be written...");

    }

    pub fn over(&mut self, x: usize, y: usize, color: Col3f64, alpha: f64) {
        let pixel_color = self.index_2d_mut(x, y);
        let result = alpha * color + (1.0 - alpha) * *pixel_color;
        *pixel_color = result;
    }
}

impl Index<usize> for Image {
    type Output = Col3f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Image {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}