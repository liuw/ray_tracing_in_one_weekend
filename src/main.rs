mod vec3;
mod ray;

use vec3::*;
use ray::*;

fn color(r: &Ray) -> Vec3 {
    if hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Vec3::new(1.0, 0.0, 0.0)
    }
    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> bool {
    let oc = r.origin() - *center;
    let a = Vec3::dot(&r.direction(), &r.direction());
    let b = 2.0 * Vec3::dot(&oc, &r.direction());
    let c = Vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    discriminant > 0.0
}

fn main() {
    let nx = 200;
    let ny = 100;

    println!("P3");
    println!("{} {}", nx, ny);
    println!("255");

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new(&origin, &(lower_left_corner + u * horizontal + v * vertical));
            let col = color(&r);

            let ir = (255.99 * col.r()) as i32;
            let ig = (255.99 * col.g()) as i32;
            let ib = (255.99 * col.b()) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
