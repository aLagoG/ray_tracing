use std::rc::Rc;

use crate::{lambertian::Lambertian, material::Material, Color, Point, Ray, Vec3};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point::ceros(),
            normal: Vec3::ceros(),
            t: 0.0,
            front_face: false,
            material: Rc::new(Lambertian::new(Color::ceros())),
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
