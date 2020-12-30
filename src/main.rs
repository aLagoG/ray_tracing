use std::io::Write;

use rand::Rng;
use raytracinginoneweekend::{random_scene, ray_color, write_to_file, Camera, Color, Point, Vec3};

fn main() {
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

    write_to_file(img_width, img_height, &buff, "test.png");
}
