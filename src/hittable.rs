use nalgebra::Vector3;

use crate::ray::Ray;

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HitRecord {
    p: Vector3<f32>,
    normal: Vector3<f32>,
    t: f32,
}

impl HitRecord {
    pub fn new(p: Vector3<f32>, normal: Vector3<f32>, t: f32) -> Self {
        Self { p, normal, t }
    }

    pub fn p(&self) -> Vector3<f32> {
        self.p
    }

    pub fn normal(&self) -> Vector3<f32> {
        self.normal
    }

    pub fn t(&self) -> f32 {
        self.t
    }
}
