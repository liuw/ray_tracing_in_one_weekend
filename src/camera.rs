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
    pub fn new(lookfrom: &Vec3, lookat: &Vec3, vup: &Vec3, vfov: f32, aspect: f32) -> Camera {
        let theta = vfov * f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = Vec3::unit_vector(*lookfrom - *lookat);
        let u = Vec3::unit_vector(Vec3::cross(&vup, &w));
        let v = Vec3::cross(&w, &u);

        Camera {
            lower_left_corner: *lookfrom - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            origin: *lookfrom,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            a: self.origin,
            b: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        }
    }
}
