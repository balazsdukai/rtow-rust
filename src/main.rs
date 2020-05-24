use std::fs::OpenOptions;
use std::io::Write;
use std::f32::MAX;
use rand::prelude::*;

pub mod vec;
pub mod ray;
pub mod hitable;
pub mod camera;
pub mod material;

use crate::vec::{Vec3};
use crate::ray::Ray;
use crate::hitable::{Sphere, HitRecord, Hitable, HitableList};
use crate::camera::Camera;
use crate::material::Material;


fn color(ray_in: &Ray, world: &HitableList, depth: i32) -> Vec3 {
    if let Some(hit_record) = world.hit(ray_in, 0.001, MAX) {
        if depth < 50 {
            let (attenuation, scattered_ray, should_scatter) = material::scatter(&hit_record.material, ray_in, &hit_record);
            if should_scatter {
                return &attenuation * &color(&scattered_ray, world, depth + 1);
            }
        }
        Vec3::new(0.0, 0.0, 0.0)
    } else {
        let unit_direction: Vec3 = ray_in.direction.unit_vector();
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

    let world = HitableList {
        list: vec![
            Box::new(Sphere {
                center: Vec3::new(0.0, 0.0, -1.0),
                radius: 0.5,
                material: Material::Lambertian { albedo: Vec3::new(0.8, 0.3, 0.3) }
            }),
            Box::new(Sphere {
                center: Vec3::new(0.0, -100.5, -1.0),
                radius: 100.0,
                material: Material::Lambertian { albedo: Vec3::new(0.8, 0.8, 0.0) }
            }),
            Box::new(Sphere {
                center: Vec3::new(1.0, 0.0, -1.0),
                radius: 0.5,
                material: Material::Metal {
                    albedo: Vec3::new(0.8, 0.6, 0.2),
                    fuzz: 0.3
                }
            }),
            Box::new(Sphere {
                center: Vec3::new(-1.0, 0.0, -1.0),
                radius: 0.5,
                material: Material::Dielectric {
                    refractive_idx: 1.5
                }
            })
        ]
    };
    let cam = Camera::new();

    let mut rng = rand::thread_rng();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            // Antialiasing: For a given pixel we have several samples (`ns`) within that pixel and
            // send rays through each of the samples. The colors of these rays are then averaged.
            for k in 0..ns {
                let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (j as f32 + rng.gen::<f32>()) / ny as f32;
                let r = cam.get_ray(u, v);
                col += color(&r, &world, 0);
            }
            // Now take the average of the color samples inside the pixel.
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
