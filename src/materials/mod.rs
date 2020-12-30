use crate::{Color, HitRecord, Ray};

mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}
