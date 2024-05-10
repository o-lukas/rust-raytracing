use nalgebra::Vector3;

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
};

pub struct Sphere {
    center: Vector3<f32>,
    radius: f32,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32, material: Box<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn center(&self) -> Vector3<f32> {
        self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn material(&self) -> &Box<dyn Material> {
        &self.material
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
        let outward_normal = (p - self.center) / self.radius;

        let mut rec = HitRecord::new(
            p,
            Vector3::default(),
            root,
            dyn_clone::clone_box(&*self.material),
        );
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }
}
