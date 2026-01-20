mod color;
mod vector;
mod image;
mod rasterization;
mod extensions;

use crate::color::Col3f64;
use crate::vector::Vec2i;
use crate::image::Image;
use crate::rasterization::{bresenham, draw_line_antialiased};

fn main() {

    let mut img: Image = Image::with_dimensions(320, 320);

    for x in (0..img.width) {
        for y in (0..img.height) {
            let r = (x as f64) / ((img.width - 1) as f64);
            let g = (y as f64) / ((img.height - 1) as f64);
            let b = 0.0;
            let reference = img.index_2d_mut(x, y);
            reference.r = r;
            reference.g = g;
            reference.b = b;
        }
    }

    let start = Vec2i::new((img.width / 2) as i32, (img.height / 2) as i32);
    let mut end = Vec2i::new(0, 80);
    bresenham(&mut img, &start, &end);
    end = Vec2i::new(0, 240);
    bresenham(&mut img, &start, &end);
    end = Vec2i::new(319, 80);
    bresenham(&mut img, &start, &end);
    end = Vec2i::new(319, 240);
    bresenham(&mut img, &start, &end);
    end = Vec2i::new(80, 0);
    bresenham(&mut img, &start, &end);
    end = Vec2i::new(240, 0);
    bresenham(&mut img, &start, &end);
    end = Vec2i::new(80, 319);
    bresenham(&mut img, &start, &end);
    end = Vec2i::new(240, 319);
    bresenham(&mut img, &start, &end);
    let start = Vec2i::new(80, 0);
    end = Vec2i::new(100, 320);
    draw_line_antialiased(&mut img, start, end, Col3f64::green());
    let start = Vec2i::new(10, 80);
    end = Vec2i::new(319, 100);
    draw_line_antialiased(&mut img, start, end, Col3f64::blue());

    img.write_to_file("output.ppm");

    println!("Done!");
}