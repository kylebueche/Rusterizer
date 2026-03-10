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
use std::rc::Rc;
use crate::color::Color;
use crate::image::*;
use crate::vector::*;
use raytracing::camera::*;
use raytracing::hittable::*;
use raytracing::implicits::sphere::*;
use raytracing::material::*;
use raytracing::bvh::*;
use crate::raytracing::texture::*;
use crate::raytracing::implicits::quad::Quad;

fn main() {
    hw3_scene3();
    //final_scene();
    //cornell_smoke()
    //simple_light();
    //quads();
    //perlin_spheres();
    //earth();
    //checkered_spheres();
    //homework_3_render_test();
}

fn hw3_scene3() {
    let mut world = HittableList::new();
    let mut noise_tex = Arc::new(NoiseTexture::new(0.05));
    let mut ground_mat = Arc::new(DiffuseLight::from_texture(noise_tex.clone()));
    let mut ground = Arc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat.clone()));

    let mut fog_container = Arc::new(Sphere::new(Vec3::new(0.0, 0.0, 2.0), 1.0, Arc::new(Dielectric::new(0.5))));
    let mut fog = Arc::new(ConstantMedium::from_texture(fog_container, 2.0, noise_tex.clone()));

    let mut fog_container2 = Arc::new(Sphere::new(Vec3::new(4.0, 1.0, 3.0), 2.0, Arc::new(Dielectric::new(3.5))));
    let mut fog2 = Arc::new(ConstantMedium::from_texture(fog_container2, 3.0, noise_tex.clone()));

    let mut fog_container3 = Arc::new(Sphere::new(Vec3::new(-5.0, 3.0, -5.0), 3.0, Arc::new(Dielectric::new(3.5))));
    let mut fog3 = Arc::new(ConstantMedium::from_texture(fog_container3, 0.45, noise_tex.clone()));

    world.add(ground);
    world.add(fog);
    world.add(fog2);
    world.add(fog3);
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Arc::new(Dielectric::new(0.5)))));
    world.add(Arc::new(Sphere::new(Vec3::new(4.0, 1.0, 3.0), 1.5, Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))))));
    world.add(Arc::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.6))))));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 3.0, -10.0), 3.0, Arc::new(Lambertian::new(Color::new(0.1, 0.7, 0.3))))));
    world.add(Arc::new(Sphere::new(Vec3::new(-3.0, 2.0, -5.0), 2.0, Arc::new(Metal::new(Color::new(1.0, 1.0, 1.0), 0.01)))));
    world.add(Arc::new(Translate::new(Arc::new(RotateY::new(block(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0), Arc::new(Dielectric::new(1.5))), 15.0)), Vec3::new(0.0, 3.0, 0.0))));
    world.add(Arc::new(Translate::new(Arc::new(RotateY::new(block(Vec3::new(-2.0, -1.0, -1.0), Vec3::new(2.0, 0.5, 1.0), Arc::new(Dielectric::new(4.0))), -30.0)), Vec3::new(0.0, 0.9, 5.0))));
    let mut camera = Camera::from_aspect_ratio(1920, 16.0 / 9.0);

    camera.samples_per_pixel = 2000;
    camera.max_depth = 40;
    camera.background = Color::new(0.0, 0.0, 0.0);

    camera.field_of_view = 60.0;
    camera.look_from = Vec3::new(0.0, 5.0, 18.0);
    camera.look_at = Vec3::new(0.0, 2.0, 0.0);
    camera.up = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;

    //let bvh = BVHNode::new(&mut world);
    let time = std::time::Instant::now();
    camera.render_threaded(&world);
    camera.viewport.write_to_file("rt.ppm");
    let time_elapsed = time.elapsed();
    println!();
    println!("Time taken to render: {} seconds", time_elapsed.as_secs_f64());
}

fn hw3_scene2() {
    let mut world = HittableList::new();
    let mut noise_tex = Arc::new(NoiseTexture::new(0.05));
    let mut ground_mat = Arc::new(Lambertian::from_texture(noise_tex.clone()));
    let mut ground = Arc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat.clone()));

    let mut fog_container = Arc::new(Sphere::new(Vec3::new(0.0, 0.0, 5.0), 1.0, Arc::new(Dielectric::new(0.3))));
    let mut fog = Arc::new(ConstantMedium::from_texture(fog_container, 1.0, noise_tex.clone()));

    let mut fog_container2 = Arc::new(Sphere::new(Vec3::new(4.0, 1.0, 3.0), 2.0, Arc::new(Dielectric::new(3.5))));
    let mut fog2 = Arc::new(ConstantMedium::from_texture(fog_container2, 1.0, noise_tex.clone()));

    let mut fog_container3 = Arc::new(Sphere::new(Vec3::new(-5.0, 3.0, -1.0), 3.0, Arc::new(Dielectric::new(3.5))));
    let mut fog3 = Arc::new(ConstantMedium::from_texture(fog_container3, 0.45, noise_tex.clone()));

    world.add(ground);
    world.add(fog);
    world.add(fog2);
    world.add(fog3);
    let mut camera = Camera::from_aspect_ratio(1920, 16.0 / 9.0);

    camera.samples_per_pixel = 1000;
    camera.max_depth = 40;
    camera.background = Color::new(0.6, 0.7, 1.0);

    camera.field_of_view = 70.0;
    camera.look_from = Vec3::new(0.0, 1.0, 10.0);
    camera.look_at = Vec3::new(0.0, 0.0, 0.0);
    camera.up = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;

    //let bvh = BVHNode::new(&mut world);
    let time = std::time::Instant::now();
    camera.render_threaded(&world);
    camera.viewport.write_to_file("rt.ppm");
    let time_elapsed = time.elapsed();
    println!();
    println!("Time taken to render: {} seconds", time_elapsed.as_secs_f64());
}


fn final_scene() {
    let mut world = HittableList::new();

    let mut boxes1 = HittableList::new();
    let ground = Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + (i as f64) * w;
            let z0 = -1000.0 + (j as f64) * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random::random_range(1.0..101.0);
            let z1 = z0 + w;
            world.add(block(Vec3::new(x0, y0, z0), Vec3::new(x1, y1, z1), ground.clone()));
        }
    }
    //let blas1 = BVHNode::new(&mut boxes1);

    //world.add(Arc::new(Translate::new(Arc::new(RotateY::new(Arc::new(blas1), 0.0)), Vec3::new(0.0, 0.0, 0.0))));
    //world.add(Arc::new(boxes1));
    let light = Arc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));
    world.add(Arc::new(Quad::new(Vec3::new(123.0, 554.0, 147.0), Vec3::new(300.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 265.0), light)));
    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    world.add(Arc::new(Sphere::new_moving(center1, center2, 50.0, sphere_material.clone())));
    world.add(Arc::new(Sphere::new(Vec3::new(260.0, 150.0, 45.0), 50.0, sphere_material.clone())));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 150.0, 145.0), 50.0, Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)))));

    let boundary =Arc::new(Sphere::new(Vec3::new(360.0, 150.0, 145.0), 70.0, Arc::new(Dielectric::new(1.5))));
    world.add(boundary.clone());
    world.add(Arc::new(ConstantMedium::new(boundary, 0.2, Color::new(0.2, 0.4, 0.9))));
    let boundary = Arc::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 5000.0, Arc::new(Dielectric::new(1.5))));
    world.add(Arc::new(ConstantMedium::new(boundary, 0.0001, Color::new(1.0, 1.0, 1.0))));

    let emat = Arc::new(Lambertian::from_texture(Arc::new(ImageTexture::new("earthmap.jpg"))));
    world.add(Arc::new(Sphere::new(Vec3::new(400.0, 200.0, 400.0), 100.0, emat)));
    let pertext = Arc::new(NoiseTexture::new(0.2));;
    world.add(Arc::new(Sphere::new(Vec3::new(220.0, 280.0, 300.0), 80.0, Arc::new(Lambertian::from_texture(pertext)))));

    let mut boxes2 = HittableList::new();
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for j in 0..ns {
        boxes2.add(Arc::new(Sphere::new(Vec3::random_range(0.0..165.0), 10.0, white.clone())));
    }

    let blas2 = Arc::new(BVHNode::new(&mut boxes2));
    world.add(Arc::new(Translate::new(Arc::new(RotateY::new(blas2, 15.0)), Vec3::new(-100.0, 270.0, 395.0))));

    let mut camera = Camera::from_aspect_ratio(800, 1.0);

    camera.samples_per_pixel = 600;
    camera.max_depth = 30;
    camera.background = Color::new(0.0, 0.0, 0.0);

    camera.field_of_view = 40.0;
    camera.look_from = Vec3::new(478.0, 278.0, -600.0);
    camera.look_at = Vec3::new(278.0, 278.0, 0.0);
    camera.up = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;

    let tlas = BVHNode::new(&mut world);
    let time = std::time::Instant::now();
    camera.render_threaded(&tlas);
    camera.viewport.write_to_file("rt.ppm");
    let time_elapsed = time.elapsed();
    println!();
    println!("Time taken to render: {} seconds", time_elapsed.as_secs_f64());
}

fn cornell_smoke() {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), green)));
    world.add(Arc::new(Quad::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), red)));
    world.add(Arc::new(Quad::new(Vec3::new(343.0, 554.0, 332.0), Vec3::new(-130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -105.0), light)));
    world.add(Arc::new(Quad::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), white.clone())));
    world.add(Arc::new(Quad::new(Vec3::new(555.0, 555.0, 555.0), Vec3::new(-555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -555.0), white.clone())));
    world.add(Arc::new(Quad::new(Vec3::new(0.0, 0.0, 555.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), white.clone())));


    let box1 = block(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 330.0, 165.0), white.clone());
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    let box2 = block(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 165.0, 165.0), white.clone());
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));

    world.add(Arc::new(ConstantMedium::new(box1, 0.01, Color::black())));
    world.add(Arc::new(ConstantMedium::new(box2, 0.01, Color::white())));

    let mut camera = Camera::from_aspect_ratio(920, 1.0);

    camera.samples_per_pixel = 500;
    camera.max_depth = 50;
    camera.background = Color::new(0.0, 0.0, 0.0);

    camera.field_of_view = 40.0;
    camera.look_from = Vec3::new(278.0, 279.0, -800.0);
    camera.look_at = Vec3::new(278.0, 278.0, 0.0);
    camera.up = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;

    let bvh = BVHNode::new(&mut world);
    let time = std::time::Instant::now();
    camera.render_threaded(&bvh);
    camera.viewport.write_to_file("rt.ppm");
    let time_elapsed = time.elapsed();
    println!();
    println!("Time taken to render: {} seconds", time_elapsed.as_secs_f64());
}

fn block(a: Vec3, b: Vec3, mat: Arc<dyn Material>) -> Arc<dyn Hittable> {
    let mut sides = HittableList::new();
    let min = Vec3::new(f64::min(a.x, b.x), f64::min(a.y, b.y), f64::min(a.z, b.z));
    let max = Vec3::new(f64::max(a.x, b.x), f64::max(a.y, b.y), f64::max(a.z, b.z));

    let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y - min.y, 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z - min.z);

    sides.add(Arc::new(Quad::new(Vec3::new(min.x, min.y, max.z), dx, dy, mat.clone())));
    sides.add(Arc::new(Quad::new(Vec3::new(max.x, min.y, max.z), -dz, dy, mat.clone())));
    sides.add(Arc::new(Quad::new(Vec3::new(max.x, min.y, min.z), -dx, dy, mat.clone())));
    sides.add(Arc::new(Quad::new(Vec3::new(min.x, min.y, min.z), dz, dy, mat.clone())));
    sides.add(Arc::new(Quad::new(Vec3::new(min.x, max.y, max.z), dx, -dz, mat.clone())));
    sides.add(Arc::new(Quad::new(Vec3::new(min.x, min.y, min.z), dx, dz, mat)));
    Arc::new(sides)
}

fn cornell_box() {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), green)));
    world.add(Arc::new(Quad::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), red)));
    world.add(Arc::new(Quad::new(Vec3::new(343.0, 554.0, 332.0), Vec3::new(-130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -105.0), light)));
    world.add(Arc::new(Quad::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), white.clone())));
    world.add(Arc::new(Quad::new(Vec3::new(555.0, 555.0, 555.0), Vec3::new(-555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -555.0), white.clone())));
    world.add(Arc::new(Quad::new(Vec3::new(0.0, 0.0, 555.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), white.clone())));


    let box1 = block(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 330.0, 165.0), white.clone());
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    let box2 = block(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 165.0, 165.0), white.clone());
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));

    world.add(box1);
    world.add(box2);

    let mut camera = Camera::from_aspect_ratio(920, 1.0);

    camera.samples_per_pixel = 30;
    camera.max_depth = 50;
    camera.background = Color::new(0.0, 0.0, 0.0);

    camera.field_of_view = 40.0;
    camera.look_from = Vec3::new(278.0, 279.0, -800.0);
    camera.look_at = Vec3::new(278.0, 278.0, 0.0);
    camera.up = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;

    let bvh = BVHNode::new(&mut world);
    let time = std::time::Instant::now();
    camera.render_threaded(&bvh);
    camera.viewport.write_to_file("rt.ppm");
    let time_elapsed = time.elapsed();
    println!();
    println!("Time taken to render: {} seconds", time_elapsed.as_secs_f64());
}

fn simple_light() {
    let mut world = HittableList::new();
    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(Lambertian::from_texture(pertext.clone())))));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, Arc::new(Lambertian::from_texture(pertext.clone())))));

    let diff_light = Arc::new(DiffuseLight::new(Color::new(4.0, 4.0, 4.0)));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 7.0, 0.0), 2.0, diff_light.clone())));
    world.add(Arc::new(Quad::new(Vec3::new(3.0, 1.0, -2.0), Vec3::new(2.0, 0.0, 0.0), Vec3::new(0.0, 2.0, 0.0), diff_light)));

    let mut camera = Camera::from_aspect_ratio(920, 16.0 / 9.0);

    camera.samples_per_pixel = 500;
    camera.max_depth = 50;
    camera.background = Color::new(0.0, 0.0, 0.0);

    camera.field_of_view = 40.0;
    camera.look_from = Vec3::new(26.0, 3.0, 6.0);
    camera.look_at = Vec3::new(0.0, 2.0, 0.0);
    camera.up = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.0;

    let time = std::time::Instant::now();
    camera.render_threaded(&world);
    camera.viewport.write_to_file("rt.ppm");
    let time_elapsed = time.elapsed();
    println!();
    println!("Time taken to render: {} seconds", time_elapsed.as_secs_f64());
}

fn quads() {
    let mut world = HittableList::new();

    // Materials
    let left_red = Arc::new(Lambertian::new(Color::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::new(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::new(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::new(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::new(Color::new(0.2, 0.8, 0.8)));

    // Quads
    world.add(Arc::new(Quad::new(Vec3::new(-3.0, -2.0, 5.0), Vec3::new(0.0, 0.0, -4.0), Vec3::new(0.0, 4.0, 0.0), left_red)));
    world.add(Arc::new(Quad::new(Vec3::new(-2.0, -2.0, 0.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 4.0, 0.0), back_green)));
    world.add(Arc::new(Quad::new(Vec3::new(3.0, -2.0, 1.0), Vec3::new(0.0, 0.0, 4.0), Vec3::new(0.0, 4.0, 0.0), right_blue)));
    world.add(Arc::new(Quad::new(Vec3::new(-2.0, 3.0, 1.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 4.0), upper_orange)));
    world.add(Arc::new(Quad::new(Vec3::new(-2.0, -3.0, 5.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -4.0), lower_teal)));

    let mut camera = Camera::from_aspect_ratio(920, 16.0/9.0);
    camera.samples_per_pixel = 500;
    camera.max_depth = 50;
    camera.field_of_view = 110.0;
    camera.look_from = Vec3::new(0.0, 0.0, 9.0);
    camera.look_at = Vec3::new(0.0, 0.0, 0.0);
    camera.up = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.0;
    camera.background = Color::new(0.70, 0.80, 1.00);
    camera.render_threaded(&world);
    camera.viewport.write_to_file("rt.ppm");
}

fn perlin_spheres() {
    let mut world = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(Lambertian::from_texture(pertext.clone())))));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, Arc::new(Lambertian::from_texture(pertext)))));

    let mut camera = Camera::from_aspect_ratio(920, 16.0 / 9.0);
    camera.samples_per_pixel = 1000;
    camera.max_depth = 150;
    camera.field_of_view = 50.0;
    camera.look_from = Vec3::new(13.0, 2.0, 3.0);
    camera.look_at = Vec3::new(0.0, 0.0, 0.0);
    camera.up = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.0;
    camera.background = Color::new(0.70, 0.80, 1.00);
    camera.render_threaded(&world);
    camera.viewport.write_to_file("rt.ppm");
}


fn earth() {
    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::from_texture(earth_texture));
    let globe = Arc::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0, earth_surface));

    let mut cam = Camera::from_aspect_ratio(920, 16.0 / 9.0);
    cam.samples_per_pixel = 50;
    cam.max_depth = 50;
    cam.field_of_view = 50.0;
    cam.look_from = Vec3::new(0.0, 0.0, 12.0);
    cam.look_at = Vec3::new(0.0, 0.0, 0.0);
    cam.up = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    cam.background = Color::new(0.70, 0.80, 1.00);
    cam.render_threaded(&*globe);
    cam.viewport.write_to_file("rt.ppm");
}

fn checkered_spheres() {
    let mut world = HittableList::new();
    let left = Color::new(0.2, 0.3, 0.1);
    let right = Color::new(0.9, 0.9, 0.9);
    let checker = Arc::new(CheckerTexture::new(0.32, left, right));
    let material = Arc::new(Lambertian::from_texture(checker));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, -10.0, 0.0), 10.0, material.clone())));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 10.0, 0.0), 10.0, material)));

    let mut camera = Camera::from_aspect_ratio(920, 16.0 / 9.0);
    camera.samples_per_pixel = 500;
    camera.max_depth = 50;
    camera.field_of_view = 50.0;
    camera.look_from = Vec3::new(13.0, 2.0, 3.0);
    camera.look_at = Vec3::new(0.0, 0.0, 0.0);
    camera.up = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 0.0;
    camera.background = Color::new(0.70, 0.80, 1.00);


    let time = std::time::Instant::now();
    camera.render_threaded(&world);
    camera.viewport.write_to_file("rt.ppm");
    let time_elapsed = time.elapsed();
    println!();
    println!("Time taken to render: {} seconds", time_elapsed.as_secs_f64());
}

fn homework_3_render_test() {
    let left = Color::new(0.2, 0.3, 0.1);
    let right = Color::new(0.9, 0.9, 0.9);
    let checker = Arc::new(CheckerTexture::new(0.32, left, right));
    let mut world = HittableList::new();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random_range(0.0..1.0);
            let fa = a as f64;
            let fb = b as f64;
            let center = Vec3 {
                x: fa + 0.9 * rand::random_range(0.0..1.0),
                y: 0.2,
                z: fb + 0.9 * rand::random_range(0.0..1.0),
            };
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2 = center + Vec3::new(0.0, random::random_range(0.0..0.5), 0.0);
                    world.add(Arc::new(Sphere::new_moving(center, center2, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5..1.0);
                    let fuzz = rand::random_range(0.0..0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3)));

    let mut camera = Camera::from_aspect_ratio(920, 16.0 / 9.0);

    camera.samples_per_pixel = 500;
    camera.max_depth = 50;

    camera.field_of_view = 35.0;
    camera.look_from = Vec3::new(13.0, 2.0, 3.0);
    camera.look_at = Vec3::new(0.0, 0.0, 0.0);
    camera.up = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;


    let ground_material = Arc::new(Lambertian::from_texture(checker));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    let world2 = BVHNode::new(&mut world);
    //let mut world3 = HittableList::new();
    //world3.add(world2);


    let time = std::time::Instant::now();
    camera.render_threaded(&world2);
    camera.viewport.write_to_file("rt.ppm");
    let time_elapsed = time.elapsed();
    println!();
    println!("Time taken to render: {} seconds", time_elapsed.as_secs_f64());
}

fn homework_2_render_3() {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Metal::new(Color::new(0.8, 0.2, 0.9), 0.1));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));
    let background_material = Arc::new(Metal::new(Color::new(0.1, 0.7, 0.1), 0.0));
    world.add(Arc::new(Sphere::new(Vec3::new(-1000.0, -300.0, -1000.0), 500.0, background_material)));
    let background_material1 = Arc::new(Lambertian::new(Color::new(0.8, 0.7, 0.1)));
    world.add(Arc::new(Sphere::new(Vec3::new(-2000.0, -700.0, -800.0), 400.0, background_material1)));
    let background_material2 = Arc::new(Lambertian::new(Color::new(1.0, 0.4, 0.1)));
    world.add(Arc::new(Sphere::new(Vec3::new(-800.0, -600.0, -2000.0), 800.0, background_material2)));

    for a in 0..500 {
        let choose_mat = rand::random_range(0.0..1.0);
        let center = Vec3 {
            x: rand::random_range(-150.0..150.0),
            y: rand::random_range(0.0..300.0),
            z: rand::random_range(-150.0..150.0),
        };
        let radius = rand::random_range(0.0..20.0);
        if (center - Vec3::new(0.0, 150.0, 0.0)).length() < 150.0 {
            if choose_mat < 0.25 {
                // diffuse
                let albedo = Color::random() * Color::random();
                let sphere_material = Arc::new(Lambertian::new(albedo));
                world.add(Arc::new(Sphere::new(center, radius, sphere_material)));
            } else if choose_mat < 0.55 {
                // metal
                let albedo = Color::random_range(0.5..1.0);
                let fuzz = rand::random_range(0.0..0.5);
                let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                world.add(Arc::new(Sphere::new(center, radius, sphere_material)));
            } else {
                // glass
                let sphere_material = Arc::new(Dielectric::new(rand::random_range(0.5..1.5)));
                world.add(Arc::new(Sphere::new(center, radius, sphere_material)));
            }
        }
    }
    let mut camera = Camera::from_aspect_ratio(1920, 16.0 / 9.0);

    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.field_of_view = 70.0;
    camera.look_from = Vec3::new(400.0, 450.0, 400.0);
    camera.look_at = Vec3::new(0.0, 150.0, 0.0);
    camera.up = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.5;
    camera.focus_dist = (camera.look_at - camera.look_from).length();

    let time = std::time::Instant::now();
    camera.render(Arc::new(world));
    camera.viewport.write_to_file("rt.ppm");
    let time_elapsed = time.elapsed();
    println!();
    println!("Time taken to render: {} seconds", time_elapsed.as_secs_f64());
}

fn homework_2_render_2() {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random_range(0.0..1.0);
            let fa = a as f64;
            let fb = b as f64;
            let center = Vec3 {
                x: fa + 0.9 * rand::random_range(-50.0..50.0),
                y: rand::random_range(0.0..50.0),
                z: fb + 0.9 * rand::random_range(-50.0..50.0),
            };
            let radius = rand::random_range(0.0..10.0);
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.3 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center, radius, sphere_material)));
                } else if choose_mat < 0.75 {
                    // metal
                    let albedo = Color::random_range(0.5..1.0);
                    let fuzz = rand::random_range(0.0..0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, radius, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, radius, sphere_material)));
                }
            }
        }
    }
    let mut camera = Camera::from_aspect_ratio(1920, 16.0 / 9.0);

    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    camera.field_of_view = 120.0;
    camera.look_from = Vec3::new(100.0, 2.0, 3.0);
    camera.look_at = Vec3::new(0.0, 30.0, 0.0);
    camera.up = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;

    let time = std::time::Instant::now();
    camera.render(Arc::new(world));
    camera.viewport.write_to_file("rt.ppm");
    let time_elapsed = time.elapsed();
    println!();
    println!("Time taken to render: {} seconds", time_elapsed.as_secs_f64());
}

fn homework_2_render_1() {
    let mut world = HittableStaticList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random_range(0.0..1.0);
            let fa = a as f64;
            let fb = b as f64;
            let center = Vec3 {
                x: fa + 0.9 * rand::random_range(0.0..1.0),
                y: 0.2,
                z: fb + 0.9 * rand::random_range(0.0..1.0),
            };
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5..1.0);
                    let fuzz = rand::random_range(0.0..0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material3));

    let mut camera = Camera::from_aspect_ratio(1920, 16.0 / 9.0);

    camera.samples_per_pixel = 250;
    camera.max_depth = 25;

    camera.field_of_view = 35.0;
    camera.look_from = Vec3::new(13.0, 2.0, 3.0);
    camera.look_at = Vec3::new(0.0, 0.0, 0.0);
    camera.up = Vec3::new(0.0, 1.0, 0.0);

    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;

    let time = std::time::Instant::now();
    camera.render_threaded(&world);
    camera.viewport.write_to_file("rt.ppm");
    let time_elapsed = time.elapsed();
    println!();
    println!("Time taken to render: {} seconds", time_elapsed.as_secs_f64());
}

/*
fn defocus_example() {
    let mut camera = Camera::from_aspect_ratio(400, 16.0/9.0);
    camera.position.z = 0.0;
    camera.front.z = -1.0;
    camera.samples_per_pixel = 50;
    camera.field_of_view = 39.0;
    camera.look_from = Vec3::new(-2.0, 2.0, 1.0);
    camera.look_at = Vec3::new(0.0, 0.0, -1.0);
    camera.up = Vec3::new(0.0, 1.0, 0.0);
    camera.defocus_angle = 10.0;
    camera.focus_dist = 3.4;
    let mut hittable_list = HittableList::new();
    let material_ground = Arc::new(Lambertian::new(Col3f64::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Col3f64::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.50));
    let material_bubble = Arc::new(Dielectric::new(1.00 / 1.50));
    let material_right = Arc::new(Metal::new(Col3f64::new(0.8, 0.6, 0.2), 1.0));
    hittable_list.add(Arc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    hittable_list.add(Arc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, material_center)));
    hittable_list.add(Arc::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    hittable_list.add(Arc::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.4, material_bubble)));
    hittable_list.add(Arc::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    camera.render(&hittable_list);
    camera.viewport.write_to_file("rt.ppm");
}
*/

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
    image_1.draw_line(a, b, Color::red(), 1.0, LineType::Bresenham);
    image_1.draw_line(b, c, Color::red(), 1.0, LineType::Bresenham);
    image_1.draw_line(c, a, Color::red(), 1.0, LineType::Bresenham);
    let mut image_2 = Image::with_dimensions(256, 256);
    image_2.draw_line(a, b, Color::red(), 1.0, LineType::Antialiased);
    image_2.draw_line(b, c, Color::red(), 1.0, LineType::Antialiased);
    image_2.draw_line(c, a, Color::red(), 1.0, LineType::Antialiased);

    let mut image_3 = Image::with_dimensions(256, 256);
    image_3.draw_triangle(a, b, c, Color::red(), 1.0, TriangleType::Scanline);

    let mut image_4 = Image::with_dimensions(256, 256);
    image_4.draw_triangle(a, b, c, Color::red(), 1.0, TriangleType::CrossAntialiased);
    image_4.draw_triangle(d, e, f, Color::green(), 1.0, TriangleType::CrossAntialiased);
    image_4.draw_triangle(g, h, i, Color::blue(), 1.0, TriangleType::CrossAntialiased);

    let mut image_5 = Image::with_dimensions(256, 256);
    image_5.draw_triangle(a, b, c, Color::red(), 0.5, TriangleType::Scanline);
    image_5.draw_triangle(d, e, f, Color::green(), 0.5, TriangleType::Scanline);
    image_5.draw_triangle(g, h, i, Color::blue(), 0.5, TriangleType::CrossAntialiased);

    let mut image_6 = Image::with_dimensions(256, 256);
    image_6.draw_point(middle, Color::green(), 0.75, 100.0, PointType::Circle);
    image_6.draw_point(middle, Color::cyan(), 0.90, 75.0, PointType::Circle);
    image_6.draw_point(middle, Color::yellow(), 0.85, 50.0, PointType::Circle);
    image_6.draw_point(middle, Color::magenta(), 0.65, 25.0, PointType::Circle);

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
        image_7.draw_line(point_list[0], point_list[1], Color::red(), 1.0, LineType::Antialiased);
        image_7.draw_line(point_list[1], point_list[2], Color::green(), 1.0, LineType::Antialiased);
        image_7.draw_line(point_list[2], point_list[3], Color::blue(), 1.0, LineType::Antialiased);
        image_7.draw_line(point_list[3], point_list[0], Color::yellow(), 1.0, LineType::Antialiased);
        image_7.draw_line(point_list[0], point_list[4], Color::magenta(), 1.0, LineType::Antialiased);
        image_7.draw_line(point_list[1], point_list[4], Color::cyan(), 1.0, LineType::Antialiased);
        image_7.draw_line(point_list[2], point_list[4], Color::cyan(), 0.5, LineType::Antialiased);
        image_7.draw_line(point_list[3], point_list[4], Color::magenta(), 0.5, LineType::Antialiased);
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
