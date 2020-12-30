use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};

pub struct HittableList {
    // TODO: check if using Box is possible instead of Rc
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Default::default(),
        }
    }

    pub fn with_objects(objects: Vec<Rc<dyn Hittable>>) -> Self {
        Self { objects }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
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
}
