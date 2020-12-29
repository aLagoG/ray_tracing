use std::{fs::File, io::BufWriter};

mod vec3;

pub fn write_gradient() {
    let nx: u32 = 200;
    let ny: u32 = 100;
    let mut buff = vec![0u8; (nx * ny * 3) as usize];

    let max_val = 255;
    let mut off = 0;
    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = (i * max_val / nx) as u8;
            let g = (j * max_val / nx) as u8;
            let b = (0.2 * max_val as f64) as u8;

            buff[off * 3] = r;
            buff[off * 3 + 1] = g;
            buff[off * 3 + 2] = b;

            off += 1;
        }
    }

    let file = File::create("gradient.png").expect("Failed creating file");
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, nx, ny);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_compression(png::Compression::Best);

    let mut writer = encoder.write_header().expect("Couldn't create writer");
    writer
        .write_image_data(&buff)
        .expect("Error while saving image data");
}
