use std::{io::Write, rc::Rc};

use rand::Rng;
use raytracinginoneweekend::{
    ray_color, write_to_file, Camera, Color, Dielectric, HittableList, Lambertian, Metal, Point,
    Sphere,
};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let img_width: u32 = 400;
    let img_height: u32 = (img_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let camera = Camera::new();

    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Rc::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        -0.4,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

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

    write_to_file(img_width, img_height, &buff, "test.png");
}
