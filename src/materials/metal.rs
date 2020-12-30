use super::Material;
use crate::{Color, Ray, Vec3};

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &crate::Ray,
        rec: &mut crate::HitRecord,
        attenuation: &mut Color,
        scattered: &mut crate::Ray,
    ) -> bool {
        let reflected = Vec3::reflect(ray.direction().unit_vector(), rec.normal);

        *scattered = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere() * self.fuzz);
        *attenuation = self.albedo;

        rec.normal.dot(scattered.direction()) > 0.0
    }
}
