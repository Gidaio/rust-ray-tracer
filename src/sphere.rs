use std::ops::Range;

use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
    vector_3::Point3,
};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Range<f64>) -> Option<HitRecord> {
        let oc = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let mut root = (h - sqrt_discriminant) / a;

        if !ray_t.contains(&root) {
            root = (h + sqrt_discriminant) / a;
            if !ray_t.contains(&root) {
                return None;
            }
        }

        let hit_point = ray.at(root);
        let outward_normal = (hit_point - self.center) / self.radius;
        let mut hit_record = HitRecord {
            point: hit_point,
            normal: (hit_point - self.center) / self.radius,
            t: root,
            front_face: false,
        };

        hit_record.set_face_normal(ray, &outward_normal);

        Some(hit_record)
    }
}
