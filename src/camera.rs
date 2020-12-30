use crate::{Point, Ray, Vec3};

pub struct Camera {
    origin: Point,
    lower_left: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: Point,
        lookat: Point,
        vup: Vec3,
        vert_fov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = vert_fov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let origin = lookfrom;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Self {
            origin,
            lower_left,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left + self.horizontal * u + self.vertical * v - self.origin,
        )
    }
}
