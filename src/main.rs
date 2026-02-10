#![allow(unused)]
mod color;
mod vector;
mod image;
mod matrix;
mod scalar;
mod mesh;
mod raytracing;
mod solid;
mod random;

use std::sync::Arc;
use crate::color::Col3f64;
use crate::image::*;
use crate::vector::*;
use raytracing::camera::*;
use raytracing::hittable::HittableList;
use raytracing::implicits::sphere::*;

fn main() {
    let mut camera = Camera::from_aspect_ratio(400, 16.0/9.0);
    camera.position.z = 0.0;
    camera.front.z = -1.0;
    let mut hittable_list = HittableList::new();
    hittable_list.add(Arc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    hittable_list.add(Arc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    camera.render(&hittable_list);
    camera.viewport.write_to_file("rt.ppm");
}

#[expect(unused)]
fn homework_one() {
    let a = Vec3::new(127.0, 20.0, 0.0);
    let b = Vec3::new(20.0, 255.0 - 20.0, 0.0);
    let c = Vec3::new(255.0 - 20.0, 255.0 - 20.0, 0.0);
    let d = Vec3::new(30.0, 30.0, 0.0);
    let e = Vec3::new(120.0, 200.0, 0.0);
    let f = Vec3::new(230.0, 130.0, 0.0);
    let g = Vec3::new(60.0, 200.0, 0.0);
    let h = Vec3::new(30.0, 86.0, 0.0);
    let i = Vec3::new(180.0, 190.0, 0.0);

    let middle = Vec3::new(127.0, 127.0, 0.0);
    let mut image_1 = Image::with_dimensions(256, 256);
    image_1.draw_line(a, b, Col3f64::red(), 1.0, LineType::Bresenham);
    image_1.draw_line(b, c, Col3f64::red(), 1.0, LineType::Bresenham);
    image_1.draw_line(c, a, Col3f64::red(), 1.0, LineType::Bresenham);
    let mut image_2 = Image::with_dimensions(256, 256);
    image_2.draw_line(a, b, Col3f64::red(), 1.0, LineType::Antialiased);
    image_2.draw_line(b, c, Col3f64::red(), 1.0, LineType::Antialiased);
    image_2.draw_line(c, a, Col3f64::red(), 1.0, LineType::Antialiased);

    let mut image_3 = Image::with_dimensions(256, 256);
    image_3.draw_triangle(a, b, c, Col3f64::red(), 1.0, TriangleType::Scanline);

    let mut image_4 = Image::with_dimensions(256, 256);
    image_4.draw_triangle(a, b, c, Col3f64::red(), 1.0, TriangleType::CrossAntialiased);
    image_4.draw_triangle(d, e, f, Col3f64::green(), 1.0, TriangleType::CrossAntialiased);
    image_4.draw_triangle(g, h, i, Col3f64::blue(), 1.0, TriangleType::CrossAntialiased);

    let mut image_5 = Image::with_dimensions(256, 256);
    image_5.draw_triangle(a, b, c, Col3f64::red(), 0.5, TriangleType::Scanline);
    image_5.draw_triangle(d, e, f, Col3f64::green(), 0.5, TriangleType::Scanline);
    image_5.draw_triangle(g, h, i, Col3f64::blue(), 0.5, TriangleType::CrossAntialiased);

    let mut image_6 = Image::with_dimensions(256, 256);
    image_6.draw_point(middle, Col3f64::green(), 0.75, 100.0, PointType::Circle);
    image_6.draw_point(middle, Col3f64::cyan(), 0.90, 75.0, PointType::Circle);
    image_6.draw_point(middle, Col3f64::yellow(), 0.85, 50.0, PointType::Circle);
    image_6.draw_point(middle, Col3f64::magenta(), 0.65, 25.0, PointType::Circle);

    let mut image_7 = Image::with_dimensions(512, 512);
    let mut point_list: Vec<Vec3> = Vec::new();
    point_list.push(2.0 * Vec3::new(10.0, 10.0, 0.0));
    point_list.push(2.0 * Vec3::new(10.0, 255.0 - 10.0, 0.0));
    point_list.push(2.0 * Vec3::new(255.0 - 10.0, 255.0 - 10.0, 0.0));
    point_list.push(2.0 * Vec3::new(255.0 - 10.0, 10.0, 0.0));
    point_list.push(2.0 * Vec3::new(127.0, 127.0, 0.0));
    for j in 0..10 {
        for i in 0..5 {
            point_list[i].translate(Vec3::new(-127.0, -127.0, 0.0));
            point_list[i].rotate(Vec3::new(0.0, 0.0, j as f64 * 20.0 / 360.0));
            point_list[i].scale(0.8);
            point_list[i].translate(Vec3::new(127.0, 127.0, 0.0));
        }
        image_7.draw_line(point_list[0], point_list[1], Col3f64::red(), 1.0, LineType::Antialiased);
        image_7.draw_line(point_list[1], point_list[2], Col3f64::green(), 1.0, LineType::Antialiased);
        image_7.draw_line(point_list[2], point_list[3], Col3f64::blue(), 1.0, LineType::Antialiased);
        image_7.draw_line(point_list[3], point_list[0], Col3f64::yellow(), 1.0, LineType::Antialiased);
        image_7.draw_line(point_list[0], point_list[4], Col3f64::magenta(), 1.0, LineType::Antialiased);
        image_7.draw_line(point_list[1], point_list[4], Col3f64::cyan(), 1.0, LineType::Antialiased);
        image_7.draw_line(point_list[2], point_list[4], Col3f64::cyan(), 0.5, LineType::Antialiased);
        image_7.draw_line(point_list[3], point_list[4], Col3f64::magenta(), 0.5, LineType::Antialiased);
    }
    image_1.write_to_file("output1.ppm");
    image_2.write_to_file("output2.ppm");
    image_3.write_to_file("output3.ppm");
    image_4.write_to_file("output4.ppm");
    image_5.write_to_file("output5.ppm");
    image_6.write_to_file("output6.ppm");
    image_7.write_to_file("output7.ppm");
}


/*
Legacy code that no longer works:

fn test_image() {

    let mut img: Image = Image::with_dimensions(320, 320);

    for x in 0..img.width {
        for y in 0..img.height {
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
    bresenham(&mut img, start, end);
    draw_line_experimental(&mut img, start, end, Col3f64::black(), 0.5);
    end = Vec2i::new(0, 240);
    bresenham(&mut img, start, end);
    draw_line_experimental(&mut img, start, end, Col3f64::black(), 0.5);
    end = Vec2i::new(319, 80);
    bresenham(&mut img, start, end);
    draw_line_experimental(&mut img, start, end, Col3f64::black(), 0.5);
    end = Vec2i::new(319, 240);
    bresenham(&mut img, start, end);
    draw_line_experimental(&mut img, start, end, Col3f64::black(), 0.5);
    end = Vec2i::new(80, 0);
    bresenham(&mut img, start, end);
    draw_line_experimental(&mut img, start, end, Col3f64::black(), 0.5);
    end = Vec2i::new(240, 0);
    bresenham(&mut img, start, end);
    draw_line_experimental(&mut img, start, end, Col3f64::black(), 0.5);
    end = Vec2i::new(80, 319);
    bresenham(&mut img, start, end);
    draw_line_experimental(&mut img, start, end, Col3f64::black(), 0.5);
    end = Vec2i::new(240, 319);
    bresenham(&mut img, start, end);
    draw_line_experimental(&mut img, start, end, Col3f64::black(), 0.5);
    let start = Vec2i::new(80, 0);
    end = Vec2i::new(100, 319);
    draw_line_antialiased(&mut img, start, end, Col3f64::green());
    let start = Vec2i::new(10, 80);
    end = Vec2i::new(319, 100);
    draw_line_antialiased(&mut img, start, end, Col3f64::blue());


    println!("Done!");

    let vec = Vec3::new(1.0, 1.0, 1.0);
    let v1: Vec2i = Vec2i::new(52, 51);
    let v2: Vec2i = Vec2i::new(107, 73);
    let v3: Vec2i = Vec2i::new(23, 101);
    draw_line_experimental(&mut img, v1, v2, Col3f64::red(), 1.0);
    draw_line_experimental(&mut img, v1, v3, Col3f64::red(), 1.0);
    draw_line_experimental(&mut img, v3, v2, Col3f64::red(), 1.0);
    scanline_triangle(&mut img, v1, v2, v3, Col3f64::green(), 0.4);
    draw_point(&mut img, v1, Col3f64::blue(), 1.0);
    draw_point(&mut img, v2, Col3f64::blue(), 1.0);
    draw_point(&mut img, v3, Col3f64::blue(), 1.0);
    draw_line_experimental(&mut img, v1, v2, Col3f64::white(), 1.0);

    let v1 = Vec2i::new(1, 1);
    let v2 = Vec2i::new(319, 319);
    for i in 0..1 {
        let time = Instant::now();
        //for i in 0..10000000 {
        //    draw_line_antialiased(&mut img, v1, v2, Col3f64::red());
        //}
        println!("Time for bresenham: {}", time.elapsed().as_millis());
        let time = Instant::now();
        for i in 0..10000 {
            draw_line_experimental(&mut img, v1, v2, Col3f64::green(), 1.0);
        }
        println!("Time for bresenham addition-only branchless: {}", time.elapsed().as_millis());
    }
    img.write_to_file("output.ppm");

}

 */
