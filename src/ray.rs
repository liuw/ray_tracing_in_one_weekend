use crate::vec3::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Ray {
    pub a: Vec3,
    pub b: Vec3,
}

impl Ray {
    pub fn new(a: &Vec3, b: &Vec3) -> Ray {
        Ray { a: *a, b: *b }
    }

    pub fn origin(self: &Ray) -> Vec3 {
        self.a
    }

    pub fn direction(self: &Ray) -> Vec3 {
        self.b
    }

    pub fn point_at_parameter(self, t: f32) -> Vec3 {
       self.a + t * self.b
    }
}
