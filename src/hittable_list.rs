use std::ops::RangeInclusive;

use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct HittableList<'a>(Vec<Box<(dyn Hittable + 'a)>>);

impl<'a> HittableList<'a> {
    pub fn new() -> Self {
        HittableList(Vec::new())
    }

    pub fn add(&mut self, object: Box<dyn Hittable + 'a>) {
        self.0.push(object);
    }
}

impl<'a> Hittable for HittableList<'a> {
    fn hit(&self, ray: &Ray, ray_t: RangeInclusive<f64>) -> Option<HitRecord> {
        let mut closest_record = None;
        let mut closest_t = *ray_t.end();

        for obj in self.0.iter() {
            if let Some(closest) = obj.hit(ray, *ray_t.start()..=closest_t) {
                closest_t = closest.t;
                closest_record = Some(closest);
            }
        }

        closest_record
    }
}
