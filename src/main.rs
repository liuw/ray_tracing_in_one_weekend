mod vec3;
mod ray;
mod hittable;
mod sphere;

use vec3::*;
use ray::*;
use hittable::*;
use sphere::*;

fn color(r: &Ray, world: &Vec<Box<&dyn Hittable>>) -> Vec3 {
    let mut rec = HitRecord { t: 0.0, p: Vec3::new(0.0,0.0,0.0), normal: Vec3::new(0.0,0.0,0.0)};

    if hit(world, r, 0.0, std::f32::MAX, &mut rec) {
        return 0.5*Vec3::new(rec.normal.x()+1.0, rec.normal.y()+1.0, rec.normal.z()+1.0)
    } else {
        let unit_direction = Vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
    }
}

fn main() {
    let nx = 800;
    let ny = 400;

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    let s1 = Sphere::new(&Vec3::new(0.0,0.0,-1.0), 0.5);
    let s2 = Sphere::new(&Vec3::new(0.0,-100.5,-1.0), 100.0);
    let world = vec![
        Box::new(&s1 as &dyn Hittable),
        Box::new(&s2 as &dyn Hittable),
    ];

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new(&origin, &(lower_left_corner + u * horizontal + v * vertical));

            let p = r.point_at_parameter(2.0);
            let col = color(&r, &world);

            let ir = (255.99 * col.r()) as i32;
            let ig = (255.99 * col.g()) as i32;
            let ib = (255.99 * col.b()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
