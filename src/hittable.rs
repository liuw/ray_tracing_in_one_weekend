use crate::material::*;
use crate::ray::*;
use crate::vec3::*;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct HitRecord<T: Material + Copy> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Option<T>,
}

pub trait Hittable<T: Material + Copy> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<T>) -> bool {
        false
    }
}

pub fn hit<T: Material + Copy>(
    list: &Vec<Box<&dyn Hittable<T>>>,
    r: &Ray,
    t_min: f32,
    t_max: f32,
    rec: &mut HitRecord<T>,
) -> bool {
    let mut temp_rec = HitRecord {
        t: 0.0,
        p: Vec3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 0.0),
        material: None,
    };
    let mut hit_anything = false;
    let mut closest_so_far = t_max;

    for h in list {
        if h.hit(r, t_min, closest_so_far, &mut temp_rec) {
            hit_anything = true;
            closest_so_far = temp_rec.t;
            *rec = temp_rec;
        }
    }

    hit_anything
}
