use crate::{material::Material, Color, Ray, Vec3};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray: &crate::Ray,
        rec: &mut crate::HitRecord,
        attenuation: &mut Color,
        scattered: &mut crate::Ray,
    ) -> bool {
        let mut direction = rec.normal + Vec3::random_unit_vector();

        if direction.approx_cero() {
            direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, direction);
        *attenuation = self.albedo;

        true
    }
}
