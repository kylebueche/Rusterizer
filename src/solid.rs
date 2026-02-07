use crate::vector::*;
pub trait Solid {
    fn is_point_inside(&self, point: Vec3) -> bool;
}