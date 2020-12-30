use std::{fs::File, io::BufWriter, rc::Rc};

mod camera;
mod dielectric;
mod hittable;
mod hittable_list;
mod lambertian;
mod material;
mod metal;
mod ray;
mod sphere;
mod vec3;

pub use camera::Camera;
pub use dielectric::Dielectric;
pub use hittable::{HitRecord, Hittable};
pub use hittable_list::HittableList;
pub use lambertian::Lambertian;
pub use metal::Metal;
pub use ray::Ray;
pub use sphere::Sphere;
pub use vec3::{Color, Point, Vec3};

pub fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
    if depth <= 0 {
        return Color::ceros();
    }

    let mut rec = HitRecord::new();
    if world.hit(ray, 0.001, std::f64::INFINITY, &mut rec) {
        let mut scattered = Ray::new(Point::ceros(), Vec3::ceros());
        let mut attenuation = Vec3::ceros();

        if Rc::clone(&rec.material).scatter(ray, &mut rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::ceros();
    }

    let unit_dir = ray.direction().unit_vector();
    let t = 0.5 * (unit_dir.y() + 1.0);
    Vec3::ones() * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

pub fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin() - *center;

    let a = ray.direction().len2();
    let half_b = oc.dot(ray.direction());
    let c = oc.len2() - radius * radius;

    let disc = half_b * half_b - a * c;

    if disc < 0.0 {
        -1.0
    } else {
        (-half_b - disc.sqrt()) / a
    }
}

pub fn write_to_file(x: u32, y: u32, data: &[u8], filename: &str) {
    let file = File::create(filename).expect("Failed creating file");
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, x, y);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_compression(png::Compression::Best);

    let mut writer = encoder.write_header().expect("Couldn't create writer");
    writer
        .write_image_data(data)
        .expect("Error while saving image data");
}
