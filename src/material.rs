use crate::{Color, HitRecord, Ray};

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        rec: &mut HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}
