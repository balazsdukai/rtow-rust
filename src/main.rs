use std::fs::OpenOptions;
use std::io::Write;

mod vec;
mod ray;
use crate::vec::{Vec3, dot};
use crate::ray::Ray;

fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> bool {
    let oc = &r.origin - &center;
    let a = dot(&r.direction, &r.direction);
    let b = 2.0 * dot(&oc, &r.direction);
    let c = dot(&oc, &oc) - radius*radius;
    let discriminant = b*b - 4_f32*a*c;
    (discriminant > 0_f32)
}

fn color(r: &Ray) -> Vec3 {
    if hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Vec3::new(1.0, 0.0, 0.0);
    }
    let unit_direction: Vec3 = r.direction.unit_vector();
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);
    &((1.0_f32 - t) * &Vec3::new(1.0, 1.0, 1.0)) + &(t * &Vec3::new(0.5, 0.7, 1.0))
}

fn main() -> std::io::Result<()> {
    let mut f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("basic.ppm")?;
    let nx: i16 = 200;
    let ny: i16 = 100;

    // let col = Vec3::new(1.0, 1.0, 1.0);

    f.write_all("P3\n".as_bytes());
    f.write_all((format!("{} {}\n", nx, ny)).as_bytes());
    f.write_all((format!("255\n")).as_bytes());
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    for j in (0..ny).rev() {
        for i in (0..nx) {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new(origin, &lower_left_corner + &(&(u*&horizontal) + &(v*&vertical)));
            let col = color(&r);
            let ir: i16 = (255.99 * col.e[0]) as i16;
            let ig: i16 = (255.99 * col.e[1]) as i16;
            let ib: i16 = (255.99 * col.e[2]) as i16;
            f.write_all((format!("{} {} {}\n", ir, ig, ib)).as_bytes());
        }
    }
    Ok(())
}
