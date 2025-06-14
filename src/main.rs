mod vector_3;

use image::{ImageBuffer, ImageFormat, Rgb};
use vector_3::Vector3;

fn main() {
    let image_width = 256;
    let image_height = 256;

    let mut img = ImageBuffer::<Rgb<u8>, _>::new(image_width, image_height);

    for (row_y, row) in img.enumerate_rows_mut() {
        println!("Scanlines remaining: {}", image_height - row_y);

        for (x, y, pixel) in row {
            let color = Vector3::new(
                x as f64 / image_width as f64,
                y as f64 / image_height as f64,
                0.0,
            );
            *pixel = color.into();
        }
    }

    img.save_with_format("out.png", ImageFormat::Png).unwrap();
}
