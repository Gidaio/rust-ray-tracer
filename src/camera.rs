use std::f64;

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
    pub samples_per_pixel: u32,
    pub max_depth: usize,
    pub vertical_fov: f64,
    pub look_from: Point3,
    pub look_at: Point3,
    pub view_up: Vector3,
    pub defocus_angle: f64,
    pub focus_distance: f64,

    image_height: u32,
    pixel_samples_scale: f64,
    center: Point3,
    upper_left_pixel_center: Point3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
    camera_u: Vector3,
    camera_v: Vector3,
    camera_w: Vector3,
    defocus_disk_u: Vector3,
    defocus_disk_v: Vector3,
}

impl Camera {
    pub fn render(&mut self, world: &impl Hittable) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        self.initialize();

        let mut img = ImageBuffer::<Rgb<u8>, _>::new(self.image_width, self.image_height);

        for (row_y, row) in img.enumerate_rows_mut() {
            println!("Scanlines remaining: {}", self.image_height - row_y);

            for (x, y, pixel) in row {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    pixel_color += ray_color(&ray, self.max_depth, world);
                }

                *pixel = (pixel_color * self.pixel_samples_scale).into();
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

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        let actual_aspect_ratio = self.image_width as f64 / self.image_height as f64;

        // Camera Stuff
        self.center = self.look_from;

        let theta = self.vertical_fov * f64::consts::PI / 180.0;
        let h = (theta / 2.0).tan();

        // Viewport Stuff
        let viewport_height = 2.0 * h * self.focus_distance;
        let viewport_width = viewport_height * actual_aspect_ratio;

        self.camera_w = (self.look_from - self.look_at).unit_vector();
        self.camera_u = self.view_up.cross(self.camera_w).unit_vector();
        self.camera_v = self.camera_w.cross(self.camera_u);

        let viewport_u = viewport_width * self.camera_u;
        let viewport_v = viewport_height * -self.camera_v;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.center - self.focus_distance * self.camera_w - viewport_u / 2.0 - viewport_v / 2.0;

        self.upper_left_pixel_center =
            viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radians = self.defocus_angle * f64::consts::PI / 180.0;
        let defocus_radius = self.focus_distance * (defocus_radians / 2.0).tan();
        self.defocus_disk_u = self.camera_u * defocus_radius;
        self.defocus_disk_v = self.camera_v * defocus_radius;
    }

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let offset = sample_square();

        let pixel_sample = self.upper_left_pixel_center
            + ((x as f64 + offset.x()) * self.pixel_delta_u)
            + ((y as f64 + offset.y()) * self.pixel_delta_v);

        let origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        Ray::new(origin, pixel_sample - origin)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vector3::random_in_unit_disc();
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }
}

fn sample_square() -> Vector3 {
    Vector3::new(
        rand::random::<f64>() - 0.5,
        rand::random::<f64>() - 0.5,
        0.0,
    )
}

fn ray_color(ray: &Ray, depth: usize, world: &impl Hittable) -> Color {
    if depth == 0 {
        Color::new(0.0, 0.0, 0.0)
    } else if let Some(hit_record) = world.hit(ray, 0.001..=f64::INFINITY) {
        if let Some((attenuation, scattered)) = hit_record.material.scatter(ray, &hit_record) {
            attenuation * ray_color(&scattered, depth - 1, world)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    } else {
        let unit_direction = ray.direction.unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
