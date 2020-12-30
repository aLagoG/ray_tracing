use rand::Rng;
use std::{
    fs::File,
    io::{BufWriter, Write},
    rc::Rc,
};

mod camera;
pub mod hittables;
pub mod materials;
mod ray;
mod vec3;

pub use camera::Camera;
pub use hittables::Hittable;
use hittables::{HitRecord, HittableList, Sphere};
pub use materials::Material;
use materials::{Dielectric, Lambertian, Metal};
pub use ray::Ray;
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

pub fn run(filename: &str) {
    let aspect_ratio = 16.0 / 9.0;
    let img_width: u32 = 1200;
    let img_height: u32 = (img_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        0.1,
        10.0,
    );

    let world = random_scene();

    let mut buff = vec![0u8; (img_width * img_height * 3) as usize];
    let mut rng = rand::thread_rng();

    let mut off = 0;
    for j in (0..img_height).rev() {
        print!("\rTodo: {:#3}", j);
        if std::io::stdout().flush().is_err() {
            eprintln!("Error flushing stdout");
        }
        for i in 0..img_width {
            let mut pixel = Color::ceros();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (img_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (img_height - 1) as f64;

                let ray = camera.get_ray(u, v);
                pixel += ray_color(&ray, &world, max_depth);
            }

            buff[off * 3] = pixel.r(samples_per_pixel);
            buff[off * 3 + 1] = pixel.g(samples_per_pixel);
            buff[off * 3 + 2] = pixel.b(samples_per_pixel);

            off += 1;
        }
    }

    println!("\nDone");

    write_to_file(img_width, img_height, &buff, filename);
}
