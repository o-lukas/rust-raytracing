use nalgebra::Vector3;

use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray,
};

pub struct Sphere {
    center: Vector3<f32>,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32) -> Self {
        Self { center, radius }
    }

    pub fn center(&self) -> Vector3<f32> {
        self.center
    }

    pub fn radius(&self) -> f32 {
        return self.radius;
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(&r.direction());
        let half_b = oc.dot(&r.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;

        let discrimant = half_b * half_b - a * c;
        if discrimant < 0.0 {
            return None;
        }
        let sqrtd = discrimant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = r.at(root);
        let normal = (p - self.center) / self.radius;
        Some(HitRecord::new(p, normal, root))
    }
}
