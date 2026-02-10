use crate::vector::*;
#[expect(unused)]
pub trait Solid {
    fn is_point_inside(&self, point: Vec3) -> bool;
}