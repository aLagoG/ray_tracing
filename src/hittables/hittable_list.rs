use std::sync::Arc;

use super::AABB;
use crate::{HitRecord, Hittable, Point};

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Default::default(),
        }
    }

    pub fn with_objects(objects: Vec<Arc<dyn Hittable>>) -> Self {
        Self { objects }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn count(&self) -> usize {
        self.objects.len()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut record = HitRecord::new();
        let mut hit = false;
        let mut closest = t_max;

        for obj in self.objects.iter() {
            if obj.hit(ray, t_min, closest, &mut record) {
                hit = true;
                closest = record.t;
                *rec = record.clone();
            }
        }

        hit
    }

    fn bounding_box(&self, output_box: &mut AABB) -> bool {
        if self.objects.is_empty() {
            return false;
        }

        let mut tmp = AABB::new(Point::ceros(), Point::ceros());
        let mut first = true;

        for object in self.objects.iter() {
            if !object.bounding_box(&mut tmp) {
                return false;
            }
            *output_box = if first {
                tmp
            } else {
                AABB::surrounding_box(output_box, &tmp)
            };
            first = false;
        }

        true
    }
}
