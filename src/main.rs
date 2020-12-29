use raytracinginoneweekend::{ray_color, write_to_file, Color, Point, Ray, Vec3};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let img_width: u32 = 400;
    let img_height: u32 = (img_width as f64 / aspect_ratio) as u32;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point::ceros();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut buff = vec![0u8; (img_width * img_height * 3) as usize];

    let mut off = 0;
    for j in (0..img_height).rev() {
        print!("\rTodo: {:#3}", j);
        for i in 0..img_width {
            let u = i as f64 / (img_width - 1) as f64;
            let v = j as f64 / (img_height - 1) as f64;

            let ray = Ray::new(origin, lower_left + horizontal * u + vertical * v - origin);

            let pixel = ray_color(&ray);
            buff[off * 3] = pixel.r();
            buff[off * 3 + 1] = pixel.g();
            buff[off * 3 + 2] = pixel.b();

            off += 1;
        }
    }

    println!("\nDone");

    write_to_file(img_width, img_height, &buff, "test.png");
}
