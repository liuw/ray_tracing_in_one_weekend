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

fn color(r: &Ray, world: &Vec<Box<&dyn Hittable>>, depth: u32) -> Vec3 {
    let mut rec = HitRecord {
        t: 0.0,
        p: Vec3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 0.0),
        material: None,
    };

    if hit(world, r, 0.001, std::f32::MAX, &mut rec) {
        let mut scattered = Ray::new(&Vec3::new(0.0, 0.0, 0.0), &Vec3::new(0.0, 0.0, 0.0));
        let mut attenuation = Vec3::new(0.0, 0.0, 0.0);
        if depth < 50
            && rec
                .material
                .clone()
                .unwrap()
                .scatter(&r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * color(&scattered, &world, depth + 1);
        } else {
            return Vec3::new(0.0, 0.0, 0.0);
        }
    } else {
        let unit_direction = Vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    }
}

#[derive(Clone)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    fn new(v: Vec3) -> Lambertian {
        Lambertian { albedo: v }
    }
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

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * Vec3::dot(v, n) * *n
}

#[derive(Clone)]
struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    fn new(v: Vec3, f: f32) -> Metal {
        let fuzz = if f < 1.0 { f } else { 1.0 };
        Metal {
            albedo: v,
            fuzz: fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(&Vec3::unit_vector(r_in.direction()), &rec.normal);
        *scattered = Ray::new(&rec.p, &(reflected + self.fuzz * random_in_unit_sphere()));
        *attenuation = self.albedo;

        Vec3::dot(&scattered.direction(), &rec.normal) > 0.0
    }
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32, refracted: &mut Vec3) -> bool {
    let uv = Vec3::unit_vector(*v);
    let dt = Vec3::dot(&uv, n);

    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        *refracted = ni_over_nt * (uv - *n * dt) - *n * discriminant.sqrt();
        return true;
    }

    false
}

#[derive(Clone)]
struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    fn new(r: f32) -> Dielectric {
        Dielectric { ref_idx: r }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let outward_normal;
        let reflected = reflect(&Vec3::unit_vector(r_in.direction()), &rec.normal);
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let ni_over_nt;
        let mut refracted = Vec3::new(0.0, 0.0, 0.0);

        if Vec3::dot(&r_in.direction(), &rec.normal) > 0.0 {
            outward_normal = -1.0 * rec.normal;
            ni_over_nt = self.ref_idx;
        } else {
            outward_normal = rec.normal;
            ni_over_nt = 1.0 / self.ref_idx;
        }

        if refract(
            &r_in.direction(),
            &outward_normal,
            ni_over_nt,
            &mut refracted,
        ) {
            *scattered = Ray::new(&rec.p, &refracted);
        } else {
            *scattered = Ray::new(&rec.p, &reflected);
            return false;
        }

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

    let s1 = Sphere::new(
        &Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Some(Box::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)))),
    );
    let s2 = Sphere::new(
        &Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Some(Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)))),
    );
    let s3 = Sphere::new(
        &Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Some(Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0))),
    );
    let s4 = Sphere::new(
        &Vec3::new(-1.0, 0.0, -1.0),
        0.5,
        Some(Box::new(Dielectric::new(1.5))),
    );

    let world = vec![
        Box::new(&s1 as &dyn Hittable),
        Box::new(&s2 as &dyn Hittable),
        Box::new(&s3 as &dyn Hittable),
        Box::new(&s4 as &dyn Hittable),
    ];

    let cam = Camera::new();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);

            for s in 0..ns {
                let u = (i as f32 + random()) / nx as f32;
                let v = (j as f32 + random()) / ny as f32;
                let r = cam.get_ray(u, v);
                col += color(&r, &world, 0);
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
