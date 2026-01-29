use crate::color::Col3f64;
use crate::image::Image;
use crate::vector::{Vec2i, Vec3};
use std::cmp::{min, max};
use std::mem::swap;
pub fn draw_point(img: &mut Image, point: Vec2i, color: Col3f64, alpha: f64) {
    img.over(point.x as usize, point.y as usize, color, alpha);
}


pub fn draw_line_experimental(img: &mut Image, start: Vec2i, end: Vec2i, color: Col3f64, alpha: f64) {
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let straight_x = if dx.abs() < dy.abs() {
        0
    } else if dx > 0 {
        1
    } else {
        -1
    };
    let straight_y = if dy.abs() < dx.abs() {
        0
    } else if dy > 0 {
        1
    } else {
        -1
    };
    let diagonal_x = if dx > 0 {
        1
    } else {
        -1
    };
    let diagonal_y = if dy > 0 {
        1
    } else {
        -1
    };

    let straight_dir = Vec2i::new(straight_x, straight_y);
    let diagonal_dir = Vec2i::new(diagonal_x, diagonal_y);
    //let cond = dx.abs() > dy.abs();
    //let d_straight = (cond as i32) * dx + (!cond as i32) * dy;
    //let d_perp = (cond as i32) * dy + (!cond as i32) * dx;
    let d_straight = if dx.abs() > dy.abs() {
        dx.abs()
    } else {
        dy.abs()
    };
    let d_perp = if dx.abs() > dy.abs() {
        dy.abs()
    } else {
        dx.abs()
    };
    let mut error = d_perp + d_perp - d_straight;
    let mut point = start;
    // I don't know why this is needed, but I needed to do this for it to line up with bresenham's original
    // I know it's correct because otherwise it misses the point and paints out of bounds
    img.over(point.x as usize, point.y as usize, color, alpha);
    point = point + straight_dir;
    while !(point.x == end.x && point.y == end.y) {
        img.over(point.x as usize, point.y as usize, color, alpha);
        error += d_perp + d_perp;
        let move_direction = if error >= 0 {
            error = error - d_straight - d_straight;
            diagonal_dir
        } else {
            straight_dir
        };
        point.x += move_direction.x;
        point.y += move_direction.y;
    }
    img.over(end.x as usize, end.y as usize, color, alpha);
}
fn plot_line_low(img: &mut Image, start: Vec2i, end: Vec2i) {
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

fn plot_line_high(img: &mut Image, start: Vec2i, end: Vec2i) {
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

pub fn bresenham(img: &mut Image, start: Vec2i, end: Vec2i) {
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
    for iy in start.y..=end.y {
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
    for ix in start.x..=end.x {
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

pub fn scanline_triangle(img: &mut Image, a: Vec2i, b: Vec2i, c: Vec2i, color: Col3f64, alpha: f64) {
    let mut top_point = a;
    let mut mid_point = b;
    let mut bottom_point = c;
    // sort so that top.y < mid.y < bottom.y
    if top_point.y > mid_point.y {
        swap(&mut top_point, &mut mid_point);
    }
    if mid_point.y > bottom_point.y {
        swap(&mut bottom_point, &mut mid_point);
    }
    if top_point.y > mid_point.y {
        swap(&mut top_point, &mut mid_point);
    }
    println!("{:?}", top_point);
    println!("{:?}", mid_point);
    println!("{:?}", bottom_point);
    // top flat-bottom triangle
    let mut dy_left = mid_point.y - top_point.y;
    let mut dx_left = mid_point.x - top_point.x;
    let mut dy_right = bottom_point.y - top_point.y;
    let mut dx_right = bottom_point.x - top_point.x;
    let mut slope_left = (dx_left as f64) / (dy_left as f64);
    let mut slope_right = (dx_right as f64) / (dy_right as f64);
    let mut x_left = top_point.x as f64;
    let mut x_right = top_point.x as f64;
    if slope_left > slope_right {
        swap(&mut slope_left, &mut slope_right);
    }
    for y in top_point.y..mid_point.y {
        let xi_left = x_left.floor() as i32;
        let xi_right = x_right.ceil() as i32;
        for x in xi_left..=xi_right {
            img.over(x as usize, y as usize, color, alpha);
        }
        x_left += slope_left;
        x_right += slope_right;
    }

    let dx_left = bottom_point.x - top_point.x;
    let dy_left = bottom_point.y - top_point.y;
    let dx_right = bottom_point.x - mid_point.x;
    let dy_right = bottom_point.y - mid_point.y;
    let mut slope_left = dx_left as f64 / dy_left as f64;
    let mut slope_right = dx_right as f64 / dy_right as f64;
    //let mut x_left = top_point.x as f64;
    //let mut x_right = mid_point.x as f64;
    if slope_right > slope_left {
        swap (&mut slope_left, &mut slope_right);
        //swap (&mut x_left, &mut x_right);
    }
    // bottom flat-top triangle
    for y in mid_point.y..=bottom_point.y {
        let xi_left = x_left.floor() as i32;
        let xi_right = x_right.ceil() as i32;
        for x in xi_left..xi_right {
            img.over(x as usize, y as usize, color, alpha);
        }
        x_left += slope_left;
        x_right += slope_right;

    }
}