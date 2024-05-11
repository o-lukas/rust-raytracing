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
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vector3<f32>, f: f32) -> Self {
        Self {
            albedo,
            fuzz: f.min(1.0),
        }
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
        let scattered = Ray::new(
            rec.p().clone(),
            reflected + self.fuzz * random_vector_in_unit_sphere(),
        );

        (scattered.direction().dot(&rec.normal()) > 0.0).then_some((self.albedo(), scattered))
    }
}

#[derive(Clone)]
pub struct Dielectric {
    ir: f32, // Index of Refraction
}

impl Dielectric {
    pub fn new(ir: f32) -> Self {
        Self { ir }
    }

    pub fn ir(&self) -> f32 {
        self.ir
    }

    pub fn refract(uv: &Vector3<f32>, n: &Vector3<f32>, etai_over_etat: f32) -> Vector3<f32> {
        let cos_theta = (-uv).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (uv + cos_theta * n);
        let r_out_parallel = (1.0 - r_out_perp.dot(&r_out_perp)).abs().sqrt() * -n;
        return r_out_perp + r_out_parallel;
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vector3<f32>, Ray)> {
        let refraction_ratio = rec.front_face().then(|| 1.0 / self.ir).unwrap_or(self.ir);

        let unit_direction = r_in.direction().normalize();
        let refracted = Dielectric::refract(&unit_direction, &rec.normal(), refraction_ratio);

        return Some((
            Vector3::new(1.0, 1.0, 1.0),
            Ray::new(rec.p().clone(), refracted),
        ));
    }
}
