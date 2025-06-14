use image::{ImageBuffer, ImageFormat, Rgb};

fn main() {
    let image_width = 256;
    let image_height = 256;

    let mut img = ImageBuffer::<Rgb<u8>, _>::new(image_width, image_height);

    for (row_y, row) in img.enumerate_rows_mut() {
        println!("Scanlines remaining: {}", image_height - row_y);

        for (x, y, pixel) in row {
            *pixel = Rgb([x as u8, y as u8, 0]);
        }
    }

    img.save_with_format("out.png", ImageFormat::Png).unwrap();
}
