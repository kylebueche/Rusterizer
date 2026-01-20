use crate::color::Col3f64;
use crate::image::Image;
use crate::vector::{Vec2f64, Vec2i};

fn plot_line_low(img: &mut Image, start: &Vec2i, end: &Vec2i) {
    let dx = end.x - start.x;
    let mut dy = end.y - start.y;
    let mut yi = 1;
    if dy < 0 {
        yi = -1;
        dy = -dy;
    }
    let mut D = (2 * dy) - dx;
    let mut y = start.y;

    for x in start.x..end.x + 1 {
        *(img.index_2d_mut(x as usize, y as usize)) = Col3f64::white();
        if D > 0 {
            y = y + yi;
            D = D + (2 * (dy - dx));
        } else {
            D = D + 2 * dy;
        }
    }
}

fn plot_line_high(img: &mut Image, start: &Vec2i, end: &Vec2i) {
    let mut dx = end.x - start.x;
    let dy = end.y - start.y;
    let mut xi = 1;
    if dx < 0 {
        xi = -1;
        dx = -dx
    }
    let mut D = (2 * dx) - dy;
    let mut x = start.x;

    for y in start.y..end.y + 1 {
        *(img.index_2d_mut(x as usize, y as usize)) = Col3f64::white();
        if D > 0 {
            x = x + xi;
            D = D + (2 * (dx - dy));
        }
        else {
            D = D + 2 * dx;
        }
    }
}

pub fn bresenham(img: &mut Image, start: &Vec2i, end: &Vec2i) {
    if (end.y - start.y).abs() < (end.x - start.x).abs() {
        if start.x > end.x {
            plot_line_low(img, end, start);
        } else {
            plot_line_low(img, start, end);
        }
    } else {
        if start.y > end.y {
            plot_line_high(img, end, start);
        } else {
            plot_line_high(img, start, end);
        }
    }
}
fn draw_vertical_line_antialiased(img: &mut Image, start: Vec2i, end: Vec2i, color: Col3f64) {
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let m = dx as f64 / dy as f64;
    let mut x = start.x as f64;
    for iy in start.y..end.y + 1{
        let ix = x as i32;
        let remainder = x - (ix as f64);
        img.over(ix as usize, iy as usize, color, 1.0 - remainder);
        img.over((ix + 1) as usize, iy as usize, color, remainder);
        x = x + m;
    }
}

fn draw_horizontal_line_antialiased(img: &mut Image, start: Vec2i, end: Vec2i, color: Col3f64) {
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let m = dy as f64 / dx as f64;
    let mut y = start.y as f64;
    for ix in start.x..end.x + 1{
        let iy = y as i32;
        let remainder = y - (iy as f64);
        img.over(ix as usize, iy as usize, color, 1.0 - remainder);
        img.over(ix as usize, (iy + 1) as usize, color, remainder);
        y = y + m;
    }
}

pub fn draw_line_antialiased(img: &mut Image, start: Vec2i, end: Vec2i, color: Col3f64) {
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    if dy.abs() > dx.abs() {
        // More vertical, plot line vertically
        let points = if start.y < end.y {
            (start, end)
        } else {
            (end, start)
        };
        draw_vertical_line_antialiased(img, points.0, points.1, color);
    } else {
        let points = if start.x < end.x {
            (start, end)
        } else {
            (end, start)
        };
        draw_horizontal_line_antialiased(img, points.0, points.1, color);
    }
}

pub fn scanline_triangle(img: &mut Image, a: Vec2i, b: Vec2i, c: Vec2i, color: Col3f64) {
    let bounding_top = if a.y > b.y && a.y > c.y {
        a.y
    } else if b.y > c.y {
        b.y
    } else {
        c.y
    }
}