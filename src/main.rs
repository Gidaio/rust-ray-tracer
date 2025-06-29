mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vector_3;

use camera::Camera;
use hittable_list::HittableList;
use image::ImageFormat;
use sphere::Sphere;
use vector_3::Point3;

fn main() {
    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;

    let img = camera.render(&world);

    img.save_with_format("out.png", ImageFormat::Png).unwrap();
}
