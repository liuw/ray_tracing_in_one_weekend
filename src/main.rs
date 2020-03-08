extern crate rand;

mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;

use camera::*;
use hittable::*;
use material::*;
use ray::*;
use sphere::*;
use vec3::*;

use rand::Rng;

fn random() -> f32 {
    let r: f32 = rand::thread_rng().gen();
    r
}

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = 2.0 * Vec3::new(random(), random(), random()) - Vec3::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            return p;
        }
    }
}

fn color(r: &Ray, world: &Vec<Box<&dyn Hittable>>) -> Vec3 {
    let mut rec = HitRecord {
        t: 0.0,
        p: Vec3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 0.0),
        material: None,
    };

    if hit(world, r, 0.001, std::f32::MAX, &mut rec) {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        return 0.5 * color(&Ray::new(&rec.p, &(target - rec.p)), &world);
    } else {
        let unit_direction = Vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    }
}

#[derive(Copy, Debug, Clone)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let target = rec.p + rec.normal + random_in_unit_sphere();
        *scattered = Ray::new(&rec.p, &(target - rec.p));
        *attenuation = self.albedo;

        true
    }
}

fn main() {
    let nx = 800;
    let ny = 400;
    let ns = 100;

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let s1 = Sphere::new(&Vec3::new(0.0, 0.0, -1.0), 0.5, None);
    let s2 = Sphere::new(&Vec3::new(0.0, -100.5, -1.0), 100.0, None);
    let world = vec![
        Box::new(&s1 as &dyn Hittable),
        Box::new(&s2 as &dyn Hittable),
    ];

    let cam = Camera::new();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);

            for s in 0..ns {
                let u = (i as f32 + random()) / nx as f32;
                let v = (j as f32 + random()) / ny as f32;
                let r = cam.get_ray(u, v);
                col += color(&r, &world);
            }
            col /= ns as f32;
            // Gamma correction
            col = Vec3::new(col.x().sqrt(), col.y().sqrt(), col.z().sqrt());

            let ir = (255.99 * col.r()) as i32;
            let ig = (255.99 * col.g()) as i32;
            let ib = (255.99 * col.b()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
