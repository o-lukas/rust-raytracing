use nalgebra::Vector3;

use crate::{hittable::HitRecord, ray::Ray};

pub trait Material: Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vector3<f32>, Ray)>;
}
