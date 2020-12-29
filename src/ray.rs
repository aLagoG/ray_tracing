use crate::{vec3::Vec3, Point};

pub struct Ray {
    origin: Point,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanity() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);

        let ray = Ray::new(a, b);

        assert!(ray.origin().approx_eq(a));
        assert!(ray.direction().approx_eq(b));
    }

    #[test]
    fn point_at() {
        let ray = Ray::new(Vec3::new(1.0, 2.0, 3.0), Vec3::new(4.0, 5.0, 6.0));
        let t = 8.0;

        let res = Vec3::new(33.0, 42.0, 51.0);

        assert!(res.approx_eq(ray.at(t)));
    }
}
