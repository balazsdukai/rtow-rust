use std::fs::OpenOptions;
use std::io::Write;

mod vec;
mod ray;
use crate::vec::{Vec3, dot};
use crate::ray::Ray;

fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> (bool, f32) {
    let oc = &r.origin - &center; // origin coordinate
    let a = dot(&r.direction, &r.direction);
    let b = 2.0 * dot(&oc, &r.direction);
    let c = dot(&oc, &oc) - radius*radius;
    let discriminant = b*b - 4_f32*a*c;
    if discriminant < 0_f32 {
        (false, -1.0) // we ray doesn't hit the sphere, no matter how negative is the discriminant
    }
    else {
        let hitpoint_1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let hitpoint_2 = (-b + discriminant.sqrt()) / (2.0 * a);
        // for checking for 0,1,2 solutions see: https://stackoverflow.com/q/15398427
        if discriminant < 0.0005 {
            // ray hits the sphere at a single point --> the "edge"
            (true, hitpoint_1)
        }
        else {
            // ray hits the sphere somewhere that is not the "edge"
            (false, hitpoint_1)
        }
    }
}

fn color(r: &Ray) -> Vec3 {
    let sphere_center = Vec3::new(0.0, 0.0, -1.0);
    let (edge, t) = hit_sphere(&sphere_center, 0.5, r); // the exact location of the hit on the surface of the sphere
    if t > 0.0 {
        if edge {
            // paint the edge red
            return Vec3::new(1.0, 0.0, 0.0);
        }
        else {
            let n: Vec3 = (&r.point_at_parameter(t) - &sphere_center).unit_vector();
            return 0.5 * &Vec3::new(n.x()+1.0, n.y()+1.0, n.z()+1.0);
        }
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
