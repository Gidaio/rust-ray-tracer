use std::ops::Range;

use crate::{
    ray::Ray,
    vector_3::{Point3, Vector3},
};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vector3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3) {
        self.front_face = ray.direction.dot(*outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: Range<f64>) -> Option<HitRecord>;
}
