use std::fs::OpenOptions;
use std::io::Write;
use std::f32::MAX;

mod vec;
mod ray;
mod hitable;
use crate::vec::{Vec3};
use crate::ray::Ray;
use crate::hitable::{Sphere, HitRecord, Hitable, HitableList};

fn color(r: &Ray, world: &HitableList) -> Vec3 {
    if let Some(hit_record) = world.hit(r, 0.0, MAX) {
        if hit_record.on_edge {
            // we are hitting the visible edge of the sphere, so paint it red
            return Vec3::new(1.0, 0.0, 0.0);
        }
        else {
            return 0.5 * &Vec3::new(hit_record.normal.x()+1.0,
                                    hit_record.normal.y()+1.0,
                                    hit_record.normal.z()+1.0);
        }
    }
    else {
        let unit_direction: Vec3 = r.direction.unit_vector();
        let t: f32 = 0.5 * (unit_direction.y() + 1.0);
        &((1.0_f32 - t) * &Vec3::new(1.0, 1.0, 1.0)) + &(t * &Vec3::new(0.5, 0.7, 1.0))
    }
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

    f.write_all("P3\n".as_bytes());
    f.write_all((format!("{} {}\n", nx, ny)).as_bytes());
    f.write_all((format!("255\n")).as_bytes());
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let world = HitableList{list: vec![
        Box::new(Sphere{center: Vec3::new(0.0, 0.0, -1.0), radius: 0.5}),
        Box::new(Sphere{center: Vec3::new(0.0, -100.5, -1.0), radius: 100.0})
    ]};
    for j in (0..ny).rev() {
        for i in (0..nx) {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new(origin, &lower_left_corner + &(&(u*&horizontal) + &(v*&vertical)));

            let p = r.point_at_parameter(2.0);
            let col = color(&r, &world);
            let ir: i16 = (255.99 * col.e[0]) as i16;
            let ig: i16 = (255.99 * col.e[1]) as i16;
            let ib: i16 = (255.99 * col.e[2]) as i16;
            f.write_all((format!("{} {} {}\n", ir, ig, ib)).as_bytes());
        }
    }
    Ok(())
}
