use std::{fs::File, io::BufWriter};

mod ray;
mod vec3;

use ray::Ray;
use vec3::Vec3;

pub fn test_part() {
    let nx: u32 = 200;
    let ny: u32 = 100;
    let mut buff = vec![0u8; (nx * ny * 3) as usize];

    let lower_left = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::origin();

    let max = 255.99;
    let mut off = 0;
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / nx as f32;

            let ray = Ray::new(origin, lower_left + horizontal * u + vertical * v);
            let color = color(&ray);

            buff[off * 3] = (color[0] * max) as u8;
            buff[off * 3 + 1] = (color[1] * max) as u8;
            buff[off * 3 + 2] = (color[2] * max) as u8;

            off += 1;
        }
    }

    write_to_file(nx, ny, &buff, "test.png");
}

pub fn color(ray: &Ray) -> Vec3 {
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
