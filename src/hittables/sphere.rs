use std::sync::Arc;

use super::AABB;
use crate::{materials::Material, HitRecord, Hittable, Point, Vec3};

pub struct Sphere {
    center: Point,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;

        let a = ray.direction().len2();
        let half_b = oc.dot(ray.direction());
        let c = oc.len2() - self.radius * self.radius;

        let disc = half_b * half_b - a * c;
        if disc < 0.0 {
            return false;
        }
        let sqrt = disc.sqrt();

        let mut root = (-half_b - sqrt) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        rec.material = Arc::clone(&self.material);

        true
    }

    fn bounding_box(&self, output_box: &mut AABB) -> bool {
        let rad = Vec3::new(self.radius, self.radius, self.radius);
        *output_box = AABB::new(self.center - rad, self.center + rad);

        true
    }
}
