use std::fs::OpenOptions;
use std::io::Write;
use std::f32::MAX;
use rand::prelude::*;

mod vec;
mod ray;
mod hitable;
mod camera;
use crate::vec::{Vec3};
use crate::ray::Ray;
use crate::hitable::{Sphere, HitRecord, Hitable, HitableList};
use crate::camera::Camera;

fn random_in_unit_sphere() -> Vec3 {
    // Rejection method algorithm for picking a random point in a unit radius sphere centered
    // at the origin. Diffuse materials have a random reflection.
    let mut p = Vec3::new(1.0, 1.0, 1.0);
    let mut rng = rand::thread_rng();
    // Try if the point is outside the sphere
    while { p.squared_length() >= 1.0 } {
        // Pick a random point in the unit cube where x,y,z range from -1 to +1
        let pt_in_unit_cube = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>());
        p = &(2.0 * &pt_in_unit_cube) - &Vec3::new(1.0, 1.0, 1.0);
    }
    p
}

fn color(r: &Ray, world: &HitableList) -> Vec3 {
    if let Some(hit_record) = world.hit(r, 0.001, MAX) {
        if hit_record.on_edge {
            // we are hitting the visible edge of the sphere, so paint it red
            return Vec3::new(1.0, 0.0, 0.0);
        }
        else {
            // Center of unit radius sphere that is tangent to the hitpoint
            let unit_center: &Vec3 = &(&hit_record.p + &hit_record.normal);
            let target: Vec3 = unit_center + &random_in_unit_sphere();
            // Diffuse material: pick a random point from the unit radius sphere that is tangent to
            // the hitpoint, and send a ray from the hitpoint 'p' to the random point.
            return 0.5 * &color( &Ray{origin:hit_record.p, direction: &target-&hit_record.p }, world);
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
    let ns: i16 = 100;

    f.write_all("P3\n".as_bytes());
    f.write_all((format!("{} {}\n", nx, ny)).as_bytes());
    f.write_all((format!("255\n")).as_bytes());

    let world = HitableList{list: vec![
        Box::new(Sphere{center: Vec3::new(0.0, 0.0, -1.0), radius: 0.5}),
        Box::new(Sphere{center: Vec3::new(0.0, -100.5, -1.0), radius: 100.0})
    ]};
    let cam = Camera::new();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for k in 0..ns {
                let mut rng = rand::thread_rng();
                let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (j as f32 + rng.gen::<f32>()) / ny as f32;
                let r = cam.get_ray(u, v);
                let p = r.point_at_parameter(2.0);
                col += color(&r, &world);
            }
            col = &col / ns as f32;
            // Apply 'gamma 2' correction --> raise the color to the power of 1/gamma
            col = Vec3::new(col.e[0].sqrt(), col.e[1].sqrt(), col.e[2].sqrt());
            let ir: i16 = (255.99 * col.e[0]) as i16;
            let ig: i16 = (255.99 * col.e[1]) as i16;
            let ib: i16 = (255.99 * col.e[2]) as i16;
            f.write_all((format!("{} {} {}\n", ir, ig, ib)).as_bytes());
        }
    }
    Ok(())
}
