use dyn_clone::DynClone;
use nalgebra::Vector3;

use crate::{hittable::HitRecord, random_vector_in_unit_sphere, ray::Ray};

pub trait Material: Sync + DynClone {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vector3<f32>, Ray)>;
}

#[derive(Clone)]
pub struct Lambertian {
    albedo: Vector3<f32>,
}

impl Lambertian {
    pub fn new(albedo: Vector3<f32>) -> Self {
        Self { albedo }
    }

    pub fn albedo(&self) -> Vector3<f32> {
        return self.albedo;
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Vector3<f32>, Ray)> {
        let mut scatter_direction = rec.normal() + random_vector_in_unit_sphere().normalize();

        // Catch degenerate scatter direction
        if almost::zero(scatter_direction.x)
            && almost::zero(scatter_direction.y)
            && almost::zero(scatter_direction.z)
        {
            scatter_direction = rec.normal();
        }

        return Some((self.albedo(), Ray::new(rec.p().clone(), scatter_direction)));
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Vector3<f32>,
}

impl Metal {
    pub fn new(albedo: Vector3<f32>) -> Self {
        Self { albedo }
    }

    pub fn albedo(&self) -> Vector3<f32> {
        return self.albedo;
    }

    fn reflect(v: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
        return v - 2.0 * v.dot(n) * n;
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vector3<f32>, Ray)> {
        let reflected = Metal::reflect(&r_in.direction().normalize(), &rec.normal());
        let scattered = Ray::new(rec.p().clone(), reflected);

        (scattered.direction().dot(&rec.normal()) > 0.0).then_some((self.albedo(), scattered))
    }
}
