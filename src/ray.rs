use nalgebra::Vector3;

pub struct Ray {
    orig: Vector3<f32>,
    dir: Vector3<f32>,
}

impl Ray {
    pub fn new(orig: Vector3<f32>, dir: Vector3<f32>) -> Self {
        Ray { orig, dir }
    }

    pub fn origin(&self) -> Vector3<f32> {
        self.orig
    }

    pub fn direction(&self) -> Vector3<f32> {
        self.dir
    }

    pub fn at(&self, t: f32) -> Vector3<f32> {
        self.orig + t * self.dir
    }
}
