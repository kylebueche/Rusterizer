use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};

// WIP Very Incomplete
#[expect(unused)]
pub trait Scalar<T>: Add<T, Output=T> + Sub<T, Output=T> + Mul<T, Output=T> + Div<T, Output=T>
+ AddAssign<T> + SubAssign<T> + MulAssign<T> + DivAssign<T> + Neg<Output=T>
+ Clone + Copy {
    fn sqrt(self) -> T;
}

impl Scalar<f32> for f32 {
    fn sqrt(self) -> f32 {self.sqrt()}
}
impl Scalar<f64> for f64 {
    fn sqrt(self) -> f64 {self.sqrt()}
}