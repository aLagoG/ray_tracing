use crate::{material::Material, Color, Ray, Vec3};

pub struct Dielectric {
    ri: f64,
}

impl Dielectric {
    pub fn new(ri: f64) -> Self {
        Self { ri }
    }

    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray: &crate::Ray,
        rec: &mut crate::HitRecord,
        attenuation: &mut crate::Color,
        scattered: &mut crate::Ray,
    ) -> bool {
        *attenuation = Color::ones();

        let refraction_ratio = if rec.front_face {
            1.0 / self.ri
        } else {
            self.ri
        };

        let unit_direction = ray.direction().unit_vector();
        let cos_theta = rec.normal.dot(-unit_direction).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction;
        if refraction_ratio * sin_theta > 1.0
            || Dielectric::reflectance(cos_theta, refraction_ratio) > rand::random()
        {
            direction = Vec3::reflect(unit_direction, rec.normal);
        } else {
            direction = Vec3::refract(unit_direction, rec.normal, refraction_ratio);
        }

        *scattered = Ray::new(rec.p, direction);

        true
    }
}
