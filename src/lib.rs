use rand::Rng;
use std::{
    fs::File,
    io::{BufWriter, Write},
    rc::Rc,
};

#[macro_use]
mod macros;

mod camera;
mod config;
pub mod hittables;
pub mod materials;
mod ray;
mod vec3;

pub use camera::Camera;
pub use config::{CameraConfig, ImgConfig, RunConfig, SceneConfig};
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

pub fn random_scene(config: &SceneConfig) -> HittableList {
    config.validate();
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let mut rng = rand::thread_rng();
    let goal_count = config.small_sphere_count as f64;
    let mut current_count = 0.0;
    let mut iterations_remainig = 484.0;
    for a in -11..11 {
        let a = a as f64;
        for b in -11..11 {
            let b = b as f64;

            let keep_prob = ((goal_count - current_count) / iterations_remainig).min(1.0);
            iterations_remainig -= 1.0;
            if float_eq!(current_count, goal_count) {
                break;
            }
            if !rng.gen_bool(keep_prob) {
                continue;
            }

            let choose_mat: f64 = rng.gen();
            let center = Point::new(a + 0.9 * rng.gen::<f64>(), 0.2, b + 0.9 * rng.gen::<f64>());

            if (center - Point::new(4.0, 0.2, 0.0)).len() > 0.9 {
                let sphere_material: Rc<dyn Material>;

                if choose_mat < config.diffuse_prob {
                    // diffuse
                    let albedo = Color::random_in_unit_cube() * Color::random_in_unit_cube();
                    sphere_material = Rc::new(Lambertian::new(albedo));
                } else if choose_mat < config.diffuse_prob + config.metal_prob {
                    // metal
                    let albedo = Color::random_in_range(0.5, 1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    sphere_material = Rc::new(Metal::new(albedo, fuzz));
                } else {
                    // glass
                    sphere_material = Rc::new(Dielectric::new(1.5));
                }

                world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                current_count += 1.0;
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

pub fn run(config: &RunConfig) {
    let RunConfig {
        img_config,
        cam_config,
        scene_config,
        filename,
        quiet,
    } = config;

    let img_height: u32 = (img_config.width as f64 / img_config.aspect_ratio) as u32;

    let camera = Camera::new(
        cam_config.lookfrom,
        cam_config.lookat,
        cam_config.vec_up,
        cam_config.vert_fov,
        img_config.aspect_ratio,
        cam_config.aperture,
        cam_config.focus_dist,
    );

    let world = random_scene(&scene_config);

    let mut buff = vec![0u8; (img_config.width * img_height * 3) as usize];
    let mut rng = rand::thread_rng();

    let mut off = 0;
    for j in (0..img_height).rev() {
        if !quiet {
            print!("\rTodo: {:#3}", j);
            if std::io::stdout().flush().is_err() {
                eprintln!("Error flushing stdout");
            }
        }
        for i in 0..img_config.width {
            let mut pixel = Color::ceros();
            for _ in 0..img_config.samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (img_config.width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (img_height - 1) as f64;

                let ray = camera.get_ray(u, v);
                pixel += ray_color(&ray, &world, img_config.max_depth);
            }

            buff[off * 3] = pixel.r(img_config.samples_per_pixel);
            buff[off * 3 + 1] = pixel.g(img_config.samples_per_pixel);
            buff[off * 3 + 2] = pixel.b(img_config.samples_per_pixel);

            off += 1;
        }
    }

    if !quiet {
        println!("\nDone");
    }

    if !quiet {
        write_to_file(img_config.width, img_height, &buff, filename);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_scene_count(config: &SceneConfig) {
        let mut values = Vec::new();
        for _ in 0..100 {
            let world = random_scene(config);
            values.push(world.count() - 4);
        }
        let avg = (values.iter().sum::<usize>() as f64 / values.len() as f64) as i64;
        assert!(
            (config.small_sphere_count as i64 - avg).abs()
                < (config.small_sphere_count / 20).max(1) as i64
        );
    }

    #[test]
    fn random_scene_count() {
        let mut conf = SceneConfig::default();
        run_scene_count(&conf);

        for c in (0..=400).step_by(50) {
            conf.small_sphere_count = c;
            run_scene_count(&conf);
        }
    }
}
