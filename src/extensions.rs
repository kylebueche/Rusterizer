use crate::image::Image;
use crate::rasterization::bresenham;
use crate::vector::{Vec2f64, Vec2i};

pub fn bresenham_f64(img: &mut Image, start: Vec2f64, end: Vec2f64) {
    // Using normalized device coordinates,
    // Bottom left pixel = (-1.0, -1.0)
    // Top right pixel = (1.0, 1.0)
    let top_left = Vec2f64::new(-1.0, 1.0);
    let top_right = Vec2f64::new(1.0, 1.0);
    let bottom_left = Vec2f64::new(-1.0, -1.0);
    let bottom_right = Vec2f64::new(1.0, -1.0);
    if in_viewport(start) && in_viewport(end) {
        let start = ndc_to_image(width, height, start);
        let end = ndc_to_image(width, height, end);
        bresenham(img, start, end);
    } else {
        let intersect_left = line_intersection(top_left, bottom_left, start, end);
        let intersect_right = line_intersection(top_right, bottom_right, start, end);
        let intersect_top = line_intersection(top_left, top_right, start, end);
        let intersect_bottom = line_intersection(bottom_left, bottom_right, start, end);
        if in_viewport(start) || in_viewport(end) {
            let new_point = if intersect_left.0 {
                intersect_left.1
            } else if intersect_right.0 {
                intersect_right.1
            } else if intersect_top.0 {
                intersect_top.1
            } else if intersect_bottom.0 {
                intersect_bottom.1
            }
            let new_point = ndc_to_image(new_point);
            if in_viewport(start) {
                let start = ndc_to_image(start);
                bresenham(img, start, new_point)
            } else if in_viewport(end) {
                let end = ndc_to_image(end);
                bresenham(img, new_point, end)
            }
        } else {
            // No points in viewport
            if
        }
    }
}

// Passed in point must be between -1.0 and 1.0 for x and y
// normalized device coordinates to image coordinates
pub fn ndc_to_image(width: usize, height: usize, point: Vec2f64) -> Vec2i {
    let xScale = (width as f64) / 2.0;
    let yScale = (height as f64) / 2.0;
    Vec2::new((1.0 + point.x) * xScale, (1.0 + point.y) * yScale)
}

// Helps with out of bounds line drawing
// Returns true if line segments intersect
// Given
// Definitions:
// Line 1: p1 <---> p2
// Line 2: p3 <---> p4
// An intersection exists iff:
// 1. p1 + s * (p2 - p1) = p3 + t * (p3 - p4)
// 2. 0.0 <= s <= 1.0
// 3. 0.0 <= t <= 1.0
//
// Derivation:
// p1 + s * (p2 - p1) = p3 + t * (p3 - p4)
// s * (p2 - p1) + t * (p3 - p4) = p3 - p1
// System of Linear Equations:
// s * (x2 - x1) + t * (x3 - x4) = x3 - x1
// s * (y2 - y1) + t * (y3 - y4) = y3 - y1
// Substitute:
// a1x + b1y = c1
// a2x + b2y = c2
// s = x
// t = y
// a1 = (x2 - x1)
// b1 = (x3 - x4)
// c1 = (x3 - x1)
// a2 = (y2 - y1)
// b2 = (y3 - y4)
// c2 = (y3 - y1)
// Apply Cramer's Rule:
// s = x = (c1b2 - c2b1) / (a1b2 - a2b1)
// t = y = (a1c2 - a2c1) / (a1b2 - a2b1)
fn line_intersection(start1: Vec2f64, end1: Vec2f64, start2: Vec2f64, end2: Vec2f64) -> (bool, Vec2f64) {
    let dir1 = end1 - start1;
    let dir2 = end2 - start2;
    let x1 = start1.x;
    let y1 = start1.y;
    let x2 = end1.x;
    let y2 = end1.y;
    let x3 = start2.x;
    let y3 = start2.y;
    let x4 = end2.x;
    let y4 = end2.y;
    let a1 = x2 - x1;
    let b1 = x3 - x4;
    let c1 = x3 - x1;
    let a2 = y2 - y1;
    let b2 = y3 - y4;
    let c2 = y3 - y1;
    let denom = (a1 * b2 - a2 * b1);
    let s = (c1 * b2 - c2 * b1) / denom;
    let t = (a1 * c2 - a2 * c1) / denom;
    let intersection_exists = !(s < 0.0 || s > 1.0 || t < 0.0 || t > 1.0 || denom == 0.0);
    let intersection = start1 + s * (end1 - start1);
    (intersection_exists, intersection)
}

fn in_viewport(point: Vec2f64) {
    !(point.x < -1.0 || point.x > 1.0 || point.y < -1.0 || point.y > 1.0)
}
