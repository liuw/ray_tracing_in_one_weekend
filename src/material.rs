use crate::hittable::*;
use crate::ray::*;
use crate::vec3::*;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord<Self>,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool
    where
        Self: Sized + Copy,
    {
        false
    }
}
