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
use material::Material;
pub use metal::Metal;
use rand::Rng;
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

pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        let a = a as f64;
        for b in -11..11 {
            let b = b as f64;

            let choose_mat: f64 = rng.gen();
            let center = Point::new(a + 0.9 * rng.gen::<f64>(), 0.2, b + 0.9 * rng.gen::<f64>());

            if (center - Point::new(4.0, 0.2, 0.0)).len() > 0.9 {
                let sphere_material: Rc<dyn Material>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random_in_unit_cube() * Color::random_in_unit_cube();
                    sphere_material = Rc::new(Lambertian::new(albedo));
                } else if choose_mat < 0.9 {
                    // metal
                    let albedo = Color::random_in_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    sphere_material = Rc::new(Metal::new(albedo, fuzz));
                } else {
                    // glass
                    sphere_material = Rc::new(Dielectric::new(1.5));
                }

                world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));
    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));
    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}
