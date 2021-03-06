use crate::hittable::*;
use crate::material::*;
use crate::ray::*;
use crate::vec3::*;

#[derive(Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f32,
    material: Option<Box<dyn Material>>,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = Vec3::dot(&r.direction(), &r.direction());
        let b = Vec3::dot(&oc, &r.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = self.material.clone();
                return true;
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center) / self.radius;
                rec.material = self.material.clone();
                return true;
            }
        }

        false
    }
}

impl Sphere {
    pub fn new(c: &Vec3, r: f32, m: Option<Box<dyn Material>>) -> Sphere {
        Sphere {
            center: *c,
            radius: r,
            material: m,
        }
    }
}
