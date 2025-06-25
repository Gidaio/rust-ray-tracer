use std::ops::Range;

use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct HittableList(Vec<Box<dyn Hittable>>);

impl HittableList {
    pub fn new() -> Self {
        HittableList(Vec::new())
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.0.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Range<f64>) -> Option<HitRecord> {
        let mut closest_record = None;
        let mut closest_t = ray_t.end;

        for obj in self.0.iter() {
            if let Some(closest) = obj.hit(ray, ray_t.start..closest_t) {
                closest_t = closest.t;
                closest_record = Some(closest);
            }
        }

        closest_record
    }
}
