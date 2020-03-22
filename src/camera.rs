use crate::ray::*;
use crate::vec3::*;
use rand::Rng;
use std::f32;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

fn random() -> f32 {
    let r: f32 = rand::thread_rng().gen();
    r
}

fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(random(), random(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
        if Vec3::dot(&p, &p) < 1.0 {
            return p;
        }
    }
}

impl Camera {
    pub fn new(
        lookfrom: &Vec3,
        lookat: &Vec3,
        vup: &Vec3,
        vfov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let theta = vfov * f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = Vec3::unit_vector(*lookfrom - *lookat);
        let u = Vec3::unit_vector(Vec3::cross(&vup, &w));
        let v = Vec3::cross(&w, &u);

        Camera {
            lower_left_corner: *lookfrom
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin: *lookfrom,
            lens_radius: aperture / 2.0,
            w: w,
            u: u,
            v: v,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray {
            a: self.origin + offset,
            b: self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset,
        }
    }
}
