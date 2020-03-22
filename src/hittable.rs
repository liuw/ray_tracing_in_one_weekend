use crate::material::*;
use crate::ray::*;
use crate::vec3::*;

#[derive(Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Option<Box<dyn Material>>,
}

pub trait Hittable {
    fn hit(&self, _r: &Ray, _t_min: f32, _t_max: f32, _rec: &mut HitRecord) -> bool {
        false
    }
}

pub fn hit(
    list: &Vec<Box<dyn Hittable>>,
    r: &Ray,
    t_min: f32,
    t_max: f32,
    rec: &mut HitRecord,
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
            *rec = temp_rec.clone();
        }
    }

    hit_anything
}
