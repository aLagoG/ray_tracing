use crate::ray::Ray;

mod aabb;
mod bvh;
mod hit_record;
mod hittable_list;
mod sphere;

pub use aabb::AABB;
pub use bvh::BVH;
pub use hit_record::HitRecord;
pub use hittable_list::HittableList;
pub use sphere::Sphere;

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, output_box: &mut AABB) -> bool;
}
