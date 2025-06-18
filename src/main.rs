mod ray;
mod vector_3;

use image::{ImageBuffer, ImageFormat, Rgb};
use ray::Ray;
use vector_3::{Color, Point3, Vector3};

fn main() {
    // Image Output Stuff
    let ideal_aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let image_height = {
        let height = (image_width as f64 / ideal_aspect_ratio).floor() as u32;
        if height < 1 { 1 } else { height }
    };

    let actual_aspect_ratio = image_width as f64 / image_height as f64;

    // Camera Stuff
    let focal_length = 1.0;
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Viewport Stuff
    let viewport_height = 2.0;
    let viewport_width = viewport_height * actual_aspect_ratio;

    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_upper_left =
        camera_center - Vector3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    let upper_left_pixel_center = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut img = ImageBuffer::<Rgb<u8>, _>::new(image_width, image_height);

    for (row_y, row) in img.enumerate_rows_mut() {
        println!("Scanlines remaining: {}", image_height - row_y);

        for (x, y, pixel) in row {
            let pixel_center = upper_left_pixel_center + x * pixel_delta_u + y * pixel_delta_v;
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&ray);
            *pixel = pixel_color.into();
        }
    }

    img.save_with_format("out.png", ImageFormat::Png).unwrap();
}

fn ray_color(ray: &Ray) -> Color {
    let unit_direction = ray.direction.unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}
