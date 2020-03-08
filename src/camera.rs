use crate::ray::*;
use crate::vec3::*;
use std::f32;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(vfov: f32, aspect: f32) -> Camera {
        let theta = vfov * f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        Camera {
            lower_left_corner: Vec3::new(-half_width, -half_height, -1.0),
            horizontal: Vec3::new(2.0 * half_width, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0 * half_height, 0.0),
            origin: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            a: self.origin,
            b: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        }
    }
}
