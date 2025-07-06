use std::ops::RangeInclusive;

use crate::{
    material::Material,
    ray::Ray,
    vector_3::{Point3, Vector3},
};

pub struct HitRecord<'a> {
    pub point: Point3,
    pub normal: Vector3,
    pub material: &'a dyn Material,
    pub t: f64,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
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
    fn hit(&self, ray: &Ray, ray_t: RangeInclusive<f64>) -> Option<HitRecord>;
}
