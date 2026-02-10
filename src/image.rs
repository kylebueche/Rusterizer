use std::fs;
use std::mem::swap;
use std::ops::{Index, IndexMut};
use crate::color::{Col3f64, Col3u8};
use crate::vector::{Vec2i, Vec3};

#[derive(Clone)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Col3f64>,
    //pub z_buffer: Vec<f64>,
    //pub use_z_buffering: bool,
}

impl Image {
    #[expect(unused)]
    pub fn new() -> Self {
        Self {
            width: 0,
            height: 0,
            data: Vec::new(),
            //z_buffer: Vec::new(),
            //use_z_buffering: false,
        }
    }
    pub fn with_dimensions(width: usize, height: usize) -> Self {
        Self {
            width: width,
            height: height,
            data: vec![Col3f64::white(); 3 * width * height],
            //z_buffer: vec![f64::NEG_INFINITY; 3 * width * height],
            //use_z_buffering: false,
        }
    }

    /*pub fn clear_z_buffer(&mut self) {
        self.z_buffer.fill(f64::NEG_INFINITY);
    }*/

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
        for y in 0..height {
            for x in 0..width {
                let col = Col3u8::from(self.index_2d(x, y).clone());
                ppm.push_str(format!("{} {} {}\n", col.r, col.g, col.b).as_str());
            }
        }

        let file_path = String::from(filename);
        fs::write(file_path, ppm).expect("File unable to be written...");

    }

    pub fn over(&mut self, x: usize, y: usize, color: Col3f64, alpha: f64) {
        if x >= self.width || y >= self.height {
            return;
        }
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

pub enum TriangleType {
    Scanline,
    CrossAntialiased,
}

pub enum LineType {
    Bresenham,
    Antialiased,
    ExperimentalBresenham,
    XiaolinWu,
}

pub enum PointType {
    Square,
    Circle,
}

impl Image { // Rasterization interface
    pub fn draw_triangle(&mut self, a: Vec3, b: Vec3, c: Vec3, color: Col3f64, alpha: f64, triangle_type: TriangleType) {
        match triangle_type {
            TriangleType::Scanline => {
                self.scanline_triangle(Vec2i::from(a), Vec2i::from(b), Vec2i::from(c), color, alpha);
            }
            TriangleType::CrossAntialiased => {
                self.draw_triangle_cross_products_antialiased(a, b, c, color, alpha);
            }
        }
    }

    pub fn draw_line(&mut self, a: Vec3, b: Vec3, color: Col3f64, alpha: f64, line_type: LineType) {
        match line_type { // may want other line drawtypes in the future
            LineType::Bresenham => {
                self.draw_line_bresenham(Vec2i::from(a), Vec2i::from(b), color, alpha);
            }
            LineType::Antialiased => {
                self.draw_line_antialiased(Vec2i::from(a), Vec2i::from(b), color, alpha);
            }
            LineType::ExperimentalBresenham => {
                self.draw_line_experimental_bresenham(Vec2i::from(a), Vec2i::from(b), color, alpha);
            }
            LineType::XiaolinWu => {
                self.draw_line_xiaolin_wu_antialiased(Vec2i::from(a), Vec2i::from(b), color, alpha);
            }
        }
    }

    pub fn draw_point(&mut self, a: Vec3, color: Col3f64, alpha: f64, radius: f64, point_type: PointType) {
        match point_type {
            PointType::Square => {
                self.draw_point_square(Vec2i::from(a), color, alpha, radius as i32);
            }
            PointType::Circle => {
                self.draw_point_circle(Vec2i::from(a), color, alpha, radius as i32);
            }
        }
    }
}

impl Image { // Rasterization details

    fn draw_point_square(&mut self, a: Vec2i, color: Col3f64, alpha: f64, radius: i32) {
        for i in (- radius + 1)..(radius - 1) {
            for j in (- radius + 1)..(radius - 1) {
                let x = (a.x + i) as usize;
                let y = (a.y + j) as usize;
                self.over(x, y, color, alpha);
            }
        }
    }
    fn draw_point_circle(&mut self, a: Vec2i, color: Col3f64, alpha: f64, radius: i32) {
        for i in (- radius + 1)..=(radius - 1) {
            for j in (- radius + 1)..=(radius - 1) {
                let x = a.x + i;
                let y = a.y + j;
                if i * i + j * j <= (radius * radius) {
                    self.over(x as usize, y as usize, color, alpha);
                }
            }
        }
    }
    fn scanline_triangle(&mut self, a: Vec2i, b: Vec2i, c: Vec2i, color: Col3f64, alpha: f64) {
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
        // top flat-bottom triangle
        let dy_left = mid_point.y - top_point.y;
        let dx_left = mid_point.x - top_point.x;
        let dy_right = bottom_point.y - top_point.y;
        let dx_right = bottom_point.x - top_point.x;
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
                self.over(x as usize, y as usize, color, alpha);
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
                self.over(x as usize, y as usize, color, alpha);
            }
            x_left += slope_left;
            x_right += slope_right;
        }
    }

    fn draw_triangle_cross_products_antialiased(&mut self, a: Vec3, b: Vec3, c: Vec3, color: Col3f64, alpha: f64) {
        // Not working right now
        let x_samples = 8;
        let y_samples = 8;
        let top_left = Vec3::new(f64::min(a.x, f64::min(b.x, c.x)), f64::min(a.y, f64::min(b.y, c.y)), 0.0);
        let bottom_right = Vec3::new(f64::max(a.x, f64::max(b.x, c.x)), f64::max(a.y, f64::max(b.y, c.y)), 0.0);
        for x in (top_left.x.floor() as i32)..=(bottom_right.x.ceil() as i32) {
            for y in (top_left.y.floor() as i32)..=(bottom_right.y.ceil() as i32) {
                let mut final_alpha = 0.0;
                for x_sample in 0..x_samples {
                    for y_sample in 0..y_samples {
                        let point = Vec3::new(x as f64 + (x_sample as f64 / x_samples as f64 + 0.5 / x_samples as f64), y as f64 + y_sample as f64 / y_samples as f64 + 0.5 / y_samples as f64, 0.0);
                        let p_to_a = a - point;
                        let p_to_b = b - point;
                        let p_to_c = c - point;
                        //let sum_cos = v1.dot(v2) + v2.dot(v3) + v3.dot(v1);
                        let a_b = p_to_a.cross(p_to_b);
                        let b_c = p_to_b.cross(p_to_c);
                        let c_a = p_to_c.cross(p_to_a);
                        if (a_b.z >= 0.0 && b_c.z >= 0.0 && c_a.z >= 0.0) || (a_b.z < 0.0 && b_c.z < 0.0 && c_a.z < 0.0) {
                            final_alpha += 1.0 / (x_samples * y_samples) as f64;
                        }/* else {
                            let (p_from, p_to): (Vec3, Vec3) = if a_b.z < 0.0 {
                                (a, b)
                            } else if b_c.z < 0.0 {
                                (b, c)
                            } else {
                                (c, a)
                            };
                            let side_dir = (p_to - p_from).normalized();
                            let distance_from_side = (point - side_dir.dot(point - p_from) * side_dir + p_from).length_squared();
                            if distance_from_side < 1.0 {
                                self.over(x as usize, y as usize, color, alpha * distance_from_side);
                            }
                        }*/
                    }
                }
                self.over(x as usize, y as usize, color, alpha * final_alpha);
            }
        }
    }

    fn draw_line_bresenham(&mut self, start: Vec2i, end: Vec2i, color: Col3f64, alpha: f64) {
        if (end.y - start.y).abs() < (end.x - start.x).abs() {
            if start.x > end.x {
                self.plot_line_low(end, start, color, alpha);
            } else {
                self.plot_line_low(start, end, color, alpha);
            }
        } else {
            if start.y > end.y {
                self.plot_line_high(end, start, color, alpha);
            } else {
                self.plot_line_high(start, end, color, alpha);
            }
        }
    }

    fn plot_line_low(&mut self, start: Vec2i, end: Vec2i, color: Col3f64, alpha: f64) {
        let dx = end.x - start.x;
        let mut dy = end.y - start.y;
        let mut yi = 1;
        if dy < 0 {
            yi = -1;
            dy = -dy;
        }
        let mut d = (2 * dy) - dx;
        let mut y = start.y;

        for x in start.x..end.x + 1 {
            self.over(x as usize, y as usize, color, alpha);
            if d > 0 {
                y = y + yi;
                d = d + (2 * (dy - dx));
            } else {
                d = d + 2 * dy;
            }
        }
    }

    fn plot_line_high(&mut self, start: Vec2i, end: Vec2i, color: Col3f64, alpha: f64) {
        let mut dx = end.x - start.x;
        let dy = end.y - start.y;
        let mut xi = 1;
        if dx < 0 {
            xi = -1;
            dx = -dx
        }
        let mut capital_d = (2 * dx) - dy;
        let mut x = start.x;

        for y in start.y..end.y + 1 {
            self.over(x as usize, y as usize, color, alpha);
            if capital_d > 0 {
                x = x + xi;
                capital_d = capital_d + (2 * (dx - dy));
            }
            else {
                capital_d = capital_d + 2 * dx;
            }
        }
    }

    fn draw_line_antialiased(&mut self, start: Vec2i, end: Vec2i, color: Col3f64, alpha: f64) {
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        if dy.abs() > dx.abs() {
            // More vertical, plot line vertically
            let points = if start.y < end.y {
                (start, end)
            } else {
                (end, start)
            };
            self.draw_vertical_line_antialiased(points.0, points.1, color, alpha);
        } else {
            let points = if start.x < end.x {
                (start, end)
            } else {
                (end, start)
            };
            self.draw_horizontal_line_antialiased(points.0, points.1, color, alpha);
        }
    }
    fn draw_vertical_line_antialiased(&mut self, start: Vec2i, end: Vec2i, color: Col3f64, alpha: f64) {
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let m = dx as f64 / dy as f64;
        let mut x = start.x as f64;
        for iy in start.y..=end.y {
            let ix = x as i32;
            let remainder = x - (ix as f64);
            self.over(ix as usize, iy as usize, color, (1.0 - remainder) * alpha);
            self.over((ix + 1) as usize, iy as usize, color, remainder * alpha);
            x = x + m;
        }
    }

    fn draw_horizontal_line_antialiased(&mut self, start: Vec2i, end: Vec2i, color: Col3f64, alpha: f64) {
        let dx = end.x - start.x;
        let dy = end.y - start.y;
        let m = dy as f64 / dx as f64;
        let mut y = start.y as f64;
        for ix in start.x..=end.x {
            let iy = y as i32;
            let remainder = y - (iy as f64);
            self.over(ix as usize, iy as usize, color, (1.0 - remainder) * alpha);
            self.over(ix as usize, (iy + 1) as usize, color, remainder * alpha);
            y = y + m;
        }
    }


    fn draw_line_experimental_bresenham(&mut self, start: Vec2i, end: Vec2i, color: Col3f64, alpha: f64) {
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
        self.over(point.x as usize, point.y as usize, color, alpha);
        point = point + straight_dir;
        while !(point.x == end.x && point.y == end.y) {
            self.over(point.x as usize, point.y as usize, color, alpha);
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
        self.over(end.x as usize, end.y as usize, color, alpha);
    }

    fn draw_line_xiaolin_wu_antialiased(&mut self, start: Vec2i, end: Vec2i, color: Col3f64, alpha: f64) {
        //todo!(); // not implemented yet. stub.
        self.over(start.x as usize, start.y as usize, color, alpha);
        self.over(end.x as usize, end.y as usize, color, alpha);
        let dy = end.y - start.y;
        let dx = end.x - start.x;
        let k = dy as f64 / dx as f64;
        let mut capital_d: i32 = 0;
        let mut overflowed;
        let d: i32 = (k * (i32::MAX as f64) + 0.5).floor() as i32;
        let mut x0 = start.x + 1;
        let mut y0 = start.y;
        let mut x1 = end.x + 1;
        let mut y1 = end.y;
        while x0 < x1 {
            (capital_d, overflowed) = capital_d.overflowing_add(d);
            if overflowed {
                y0 = y0 + 1;
                y1 = y1 - 1;
            }
            let alpha_1 = capital_d as f64 / 2_i32.pow(32 - 8) as f64;
            self.over(x0 as usize, y0 as usize, color, alpha_1 * alpha);
            self.over(x1 as usize, y1 as usize, color, alpha_1 * alpha);
            self.over(x0 as usize, (y0 + 1) as usize, color, (1.0 - alpha_1) * alpha);
            self.over(x1 as usize, (y1 - 1) as usize, color, (1.0 - alpha_1) * alpha);
            x0 = x0 + 1;
            x1 = x1 - 1;
        }
    }
}