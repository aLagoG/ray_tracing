use crate::{Point, Ray};

#[derive(Clone, Copy)]
pub struct AABB {
    pub max: Point,
    pub min: Point,
}

impl AABB {
    pub fn new(min: Point, max: Point) -> Self {
        Self { max, min }
    }

    pub fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for a in 0..3 {
            let mut t0 = (self.min[a] - ray.origin()[a]) / ray.direction()[a];
            let mut t1 = (self.max[a] - ray.origin()[a]) / ray.direction()[a];
            if t0 > t1 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_min = t_min.max(t0);
            t_max = t_max.min(t1);
            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    pub fn surrounding_box(b0: &AABB, b1: &AABB) -> AABB {
        let small = Point::new(
            b0.min.x().min(b1.min.x()),
            b0.min.y().min(b1.min.y()),
            b0.min.z().min(b1.min.z()),
        );
        let big = Point::new(
            b0.max.x().max(b1.max.x()),
            b0.max.y().max(b1.max.y()),
            b0.max.z().max(b1.max.z()),
        );

        AABB::new(small, big)
    }
}
