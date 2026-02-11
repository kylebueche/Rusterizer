use crate::color::Col3f64;
use crate::random::{random_cosine_direction, random_on_unit_sphere_above_normal, random_unit_vector};
use crate::raytracing::hittable::*;
use crate::raytracing::ray::*;

pub trait Material {
    fn scatter(&self, ray_in: Ray, hit_record: &HitRecord, attenuation: &mut Col3f64, scattered: &mut Ray) -> bool;
}

pub struct Lambertian {
    pub albedo: Col3f64,
}

impl Lambertian {
    #[inline]
    pub fn new(albedo: Col3f64) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    #[inline]
    fn scatter(&self, ray_in: Ray, hit_record: &HitRecord, attenuation: &mut Col3f64, scattered: &mut Ray) -> bool {
        //let scatter_direction = random_on_unit_sphere_above_normal(hit_record.normal);
        let scatter_direction = random_cosine_direction(hit_record.normal);
        *scattered = Ray::new(hit_record.point, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    pub albedo: Col3f64,
    pub fuzz: f64,
}

impl Metal {
    #[inline]
    pub fn new(albedo: Col3f64, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    #[inline]
    fn scatter(&self, ray_in: Ray, hit_record: &HitRecord, attenuation: &mut Col3f64, scattered: &mut Ray) -> bool {
        let mut reflected = ray_in.direction.reflect(hit_record.normal);
        reflected = reflected.normalized() + (self.fuzz * random_unit_vector());
        *scattered = Ray::new(hit_record.point, reflected);
        *attenuation = self.albedo;
        scattered.direction.dot(hit_record.normal) > 0.0
    }
}

pub struct Dielectric {
    pub refraction_index: f64,
}

impl Dielectric {
    #[inline]
    pub fn new(refraction_index: f64) -> Self {
        Dielectric { refraction_index }
    }

    // Schlick's approximation
    #[inline]
    pub fn reflectance(cos_theta: f64, refractive_index: f64) -> f64 {
        let mut r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powi(1.0 - cos_theta, 5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: Ray, hit_record: &HitRecord, attenuation: &mut Col3f64, scattered: &mut Ray) -> bool {
        *attenuation = Col3f64::new(1.0, 1.0, 1.0);
        let refractive_index = if hit_record.front_face {
            1.0 / self.refraction_index // entering from air
        } else {
            self.refraction_index // exiting into air
        };
        let unit_direction = ray_in.direction.normalized();

        let cos_theta = f64::min((-unit_direction).dot(hit_record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refractive_index * sin_theta > 1.0;
        let direction = if cannot_refract || Dielectric::reflectance(cos_theta, refractive_index) > rand::random_range(0.0..1.0) {
            unit_direction.reflect(hit_record.normal)
        } else {
            unit_direction.refract(hit_record.normal, refractive_index)
        };

        *scattered = Ray::new(hit_record.point, direction);
        true
    }
}