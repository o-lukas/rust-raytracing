use std::rc::Rc;

use nalgebra::Vector3;

use crate::ray::Ray;

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HitRecord {
    p: Vector3<f32>,
    normal: Vector3<f32>,
    t: f32,
    front_face: bool,
}

impl HitRecord {
    pub fn new(p: Vector3<f32>, normal: Vector3<f32>, t: f32) -> Self {
        Self {
            p,
            normal,
            t,
            front_face: false,
        }
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

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vector3<f32>) {
        self.front_face = r.direction().dot(&outward_normal) < 0.0;
        self.normal = outward_normal.clone();
        if !self.front_face {
            self.normal = -self.normal;
        }
    }
}

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut rec: Option<HitRecord> = None;

        for object in self.objects.iter() {
            if let Some(temp_rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                rec = Some(temp_rec);
            }
        }

        return rec;
    }
}
