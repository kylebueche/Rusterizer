use crate::vector::*;

#[inline]
pub fn random_vector_in_unit_sphere() -> Vec3 {
    let mut sample = Vec3 {
        x: rand::random_range(-1.0..=1.0),
        y: rand::random_range(-1.0..=1.0),
        z: rand::random_range(-1.0..=1.0),
    };
    loop {
        if (f64::EPSILON..1.0).contains(&sample.length_squared()) {
            sample = Vec3 {
                x: rand::random_range(-1.0..=1.0),
                y: rand::random_range(-1.0..=1.0),
                z: rand::random_range(-1.0..=1.0),
            };
        }
        else {
            break;
        }
    }
    sample
}

#[inline]
pub fn random_unit_vector() -> Vec3 {
    random_vector_in_unit_sphere().normalized()
}

#[inline]
pub fn random_on_hemisphere(normal: Vec3) -> Vec3 {
    let on_unit_sphere = random_unit_vector();
    let on_hemisphere = if on_unit_sphere.dot(normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    };
    on_hemisphere
}

#[inline]
pub fn random_on_unit_sphere_above_normal(normal: Vec3) -> Vec3 {
    let mut on_unit_sphere = random_unit_vector();
    let mut on_unit_above_normal = on_unit_sphere + normal;
    loop  {
        if on_unit_above_normal.length_squared() > f64::EPSILON {
            break;
        }
        on_unit_sphere = random_unit_vector();
        on_unit_above_normal = on_unit_sphere + normal;
    }
    on_unit_above_normal
}

#[inline]
pub fn random_in_unit_sphere_above_normal(normal: Vec3) -> Vec3 {
    let mut in_unit_sphere = random_vector_in_unit_sphere();
    let mut in_unit_above_normal = in_unit_sphere + normal;
    loop  {
        if in_unit_above_normal.length_squared() > f64::EPSILON {
            break;
        }
        in_unit_sphere = random_unit_vector();
        in_unit_above_normal = in_unit_sphere + normal;
    }
    in_unit_above_normal
}

#[inline]
pub fn sample_square_3d(top_left: Vec3, du: Vec3, dv: Vec3) -> Vec3 {
    top_left + rand::random_range(0.0..1.0) * du + rand::random_range(0.0..1.0) * dv
}

#[inline]
pub fn sample_square() -> Vec3{
    Vec3::new(rand::random_range(-0.5..0.5),rand::random_range(-0.5..0.5), 0.0)
}