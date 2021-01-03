use rand::{thread_rng, Rng};
use std::sync::Arc;

use super::{Hittable, HittableList, AABB};
use crate::Point;

pub struct BVH {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl BVH {
    pub fn new(objects: &mut [Arc<dyn Hittable>]) -> Self {
        let axis = thread_rng().gen_range(0..3);
        let key = move |a: &Arc<dyn Hittable>| {
            let mut bbox_a = AABB::new(Point::ceros(), Point::ceros());
            if !a.bounding_box(&mut bbox_a) {
                panic!("No bounding box on BVH creation");
            }
            bbox_a.min[axis].round() as i64
        };

        let left;
        let right;

        match objects.len() {
            1 => {
                left = objects[0].clone();
                right = objects[0].clone();
            }
            2 => {
                if key(&objects[0]) < key(&objects[1]) {
                    left = objects[0].clone();
                    right = objects[1].clone();
                } else {
                    left = objects[1].clone();
                    right = objects[0].clone();
                }
            }
            _ => {
                objects.sort_by_cached_key(key);
                let mid = objects.len() / 2;
                left = Arc::new(BVH::new(&mut objects[0..mid]));
                right = Arc::new(BVH::new(&mut objects[mid..]));
            }
        }

        let mut left_box = AABB::new(Point::ceros(), Point::ceros());
        let mut right_box = AABB::new(Point::ceros(), Point::ceros());

        if !left.bounding_box(&mut left_box) || !right.bounding_box(&mut right_box) {
            panic!("No bounding box on BVH construction");
        }

        let bbox = AABB::surrounding_box(&left_box, &right_box);

        Self { left, right, bbox }
    }

    pub fn from_hittable_list(list: &mut HittableList) -> Self {
        Self::new(&mut list.objects[..])
    }
}

impl Hittable for BVH {
    fn hit(&self, ray: &crate::Ray, t_min: f64, t_max: f64, rec: &mut super::HitRecord) -> bool {
        if !self.bbox.hit(ray, t_min, t_max) {
            return false;
        }

        let hit_left = self.left.hit(ray, t_min, t_max, rec);
        let hit_right = self
            .right
            .hit(ray, t_min, if hit_left { rec.t } else { t_max }, rec);

        hit_left || hit_right
    }

    fn bounding_box(&self, output_box: &mut AABB) -> bool {
        *output_box = self.bbox;
        true
    }
}
