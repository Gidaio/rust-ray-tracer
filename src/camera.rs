use image::{ImageBuffer, Rgb};

use crate::{
    hittable::Hittable,
    ray::Ray,
    vector_3::{Color, Point3, Vector3},
};

#[derive(Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,

    image_height: u32,
    center: Point3,
    upper_left_pixel_center: Point3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
}

impl Camera {
    pub fn render(&mut self, world: &impl Hittable) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        self.initialize();

        let mut img = ImageBuffer::<Rgb<u8>, _>::new(self.image_width, self.image_height);

        for (row_y, row) in img.enumerate_rows_mut() {
            println!("Scanlines remaining: {}", self.image_height - row_y);

            for (x, y, pixel) in row {
                let pixel_center =
                    self.upper_left_pixel_center + x * self.pixel_delta_u + y * self.pixel_delta_v;
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);
                let pixel_color = Self::ray_color(&ray, world);
                *pixel = pixel_color.into();
            }
        }

        img
    }

    fn initialize(&mut self) {
        // Image Output Stuff
        self.image_height = {
            let height = (self.image_width as f64 / self.aspect_ratio).floor() as u32;
            if height < 1 { 1 } else { height }
        };

        let actual_aspect_ratio = self.image_width as f64 / self.image_height as f64;

        // Camera Stuff
        let focal_length = 1.0;
        self.center = Point3::new(0.0, 0.0, 0.0);

        // Viewport Stuff
        let viewport_height = 2.0;
        let viewport_width = viewport_height * actual_aspect_ratio;

        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left = self.center
            - Vector3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;

        self.upper_left_pixel_center =
            viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(ray: &Ray, world: &impl Hittable) -> Color {
        if let Some(hit_record) = world.hit(ray, 0.0..f64::INFINITY) {
            0.5 * (hit_record.normal + Color::new(1.0, 1.0, 1.0))
        } else {
            let unit_direction = ray.direction.unit_vector();
            let a = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
        }
    }
}
