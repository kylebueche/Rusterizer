use crate::vector::{Vec3};
use crate::image::*;
use crate::color::Col3f64;

#[expect(unused)]
pub enum DrawMode {
    Points,
    Lines,
    Triangles,
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub indices: Vec<usize>,

    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,

    pub color: Col3f64,
}

fn lerp(t: f64, a: f64, b: f64) -> f64 {
    t * (a - b) + a
}

impl Mesh {
    pub fn new(vertices: Vec<Vec3>, normals: Vec<Vec3>, indices: Vec<usize>) -> Self {
        Self {
            vertices: vertices,
            normals: normals,
            indices: indices,
            position: Vec3::new(0.0, 0.0, 0.0),
            rotation: Vec3::new(0.0, 0.0, 0.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
            color: Col3f64::new(1.0, 1.0, 1.0),
        }
    }

    pub fn uv_sphere(radius: f64, u_segments: usize, v_segments: usize) -> Self {
        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut indices: Vec<usize> = Vec::new();
        for u in 0..u_segments {
            for v in 0..v_segments {
                let mut point = Vec3::new(radius, 0.0, 0.0);
                let u_01 = (u as f64) / (u_segments as f64);
                let v_01 = (v as f64) / (v_segments as f64);
                let rotation_y_axis = lerp(u_01, 0.0, 2.0 * std::f64::consts::PI);
                let rotation_z_axis = lerp(v_01, - 0.5 * std::f64::consts::PI, 0.5 * std::f64::consts::PI);
                point.rotate(Vec3::new(0.0, rotation_y_axis, rotation_z_axis));
                vertices.push(point);
                normals.push(point.normalized());

                //let wrapped_u = u % (u_segments - 1);
                //let wrapped_v = v % (v_segments - 1);
                let top_left = u * v_segments + v;
                let top_right = (u + 1) * v_segments + v;
                let bottom_left = u * v_segments + (v + 1);
                let bottom_right = (u + 1) * v_segments + (v + 1);

                indices.push(top_left);
                indices.push(top_right);
                indices.push(bottom_right);
                indices.push(bottom_right);
                indices.push(bottom_left);
                indices.push(top_left);

            }
        }
        Mesh::new(vertices, normals, indices)
    }

    pub fn rasterize(&self, img: &mut Image, draw_mode: DrawMode, draw_normals: bool) {
        if draw_normals {
            for i in 0..self.indices.len() {
                let point = self.vertices[i].scaled_non_uniform(self.scale).rotated(self.rotation).translated(self.position);
                let normal = self.normals[i].scaled_non_uniform(self.scale).rotated(self.rotation).translated(self.position).normalized();
                img.draw_line(point, point + normal, self.color, 1.0, LineType::Antialiased);
            }
        }
        match draw_mode {
            DrawMode::Points => {
                for i in 0..self.indices.len() {
                    let point = self.vertices[i].scaled_non_uniform(self.scale).rotated(self.rotation).translated(self.position);
                    img.draw_point(point, self.color, 1.0, 1.0, PointType::Square);
                }
            }
            DrawMode::Lines => {
                for i in (0..self.indices.len()).step_by(3) {
                    let index1 = i;
                    let index2 = i + 1;
                    let index3 = i + 2;
                    let point1 = self.vertices[index1].scaled_non_uniform(self.scale).rotated(self.rotation).translated(self.position);
                    let point2 = self.vertices[index2].scaled_non_uniform(self.scale).rotated(self.rotation).translated(self.position);
                    let point3 = self.vertices[index3].scaled_non_uniform(self.scale).rotated(self.rotation).translated(self.position);
                    img.draw_line(point1, point2, self.color, 1.0, LineType::Antialiased);
                    img.draw_line(point2, point3, self.color, 1.0, LineType::Antialiased);
                    img.draw_line(point3, point1, self.color, 1.0, LineType::Antialiased);
                }
            },
            DrawMode::Triangles => {
                for i in (0..self.indices.len()).step_by(3) {
                    let index1 = i;
                    let index2 = i + 1;
                    let index3 = i + 2;
                    let point1 = self.vertices[index1].scaled_non_uniform(self.scale).rotated(self.rotation).translated(self.position);
                    let point2 = self.vertices[index2].scaled_non_uniform(self.scale).rotated(self.rotation).translated(self.position);
                    let point3 = self.vertices[index3].scaled_non_uniform(self.scale).rotated(self.rotation).translated(self.position);
                    img.draw_triangle(point1, point2, point3, self.color, 1.0, TriangleType::Scanline);
                }
            }
        }
    }
}