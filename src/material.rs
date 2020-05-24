use rand::prelude::*;

use crate::ray::Ray;
use crate::hitable::HitRecord;
use crate::vec::{Vec3, dot};

#[derive(Clone)]
pub enum Material {
    // The enums use anyonymous structs inside them to store data
    Lambertian {
        albedo: Vec3
    },
    Metal {
        albedo: Vec3,
        fuzz: f32
    },
    Dielectric {
        refractive_idx: f32
    }
}

pub fn scatter(material: &Material, ray_in: &Ray, hit_record: &HitRecord) -> (Vec3, Ray, bool) {
    match *material {
        Material::Lambertian { albedo } => {
            // Center of unit radius sphere that is tangent to the hitpoint
            let unit_center: Vec3 = &hit_record.p + &hit_record.normal;
            let target: Vec3 = &unit_center + &random_in_unit_sphere();
            // Diffuse material: pick a random point from the unit radius sphere that is tangent to
            // the hitpoint, and send a ray from the hitpoint 'p' to the random point.
            let scattered_ray = Ray { origin: hit_record.p, direction: &target - &hit_record.p };
            let attenuation = albedo;
            let should_scatter = true;
            (attenuation, scattered_ray, should_scatter)
        }
        Material::Metal { albedo, fuzz } => {
            let reflected: Vec3 = reflect(&ray_in.direction.unit_vector(), &hit_record.normal);
            let scattered_ray = Ray { origin: hit_record.p, direction: &reflected + &(fuzz * &random_in_unit_sphere())};
            let attenuation = albedo;
            let should_scatter = dot(&scattered_ray.direction, &hit_record.normal) > 0.0;
            (attenuation, scattered_ray, should_scatter)
        }
        Material::Dielectric {refractive_idx} => {
            let reflected: Vec3 = reflect(&ray_in.direction, &hit_record.normal);
            let attenuation = Vec3::new(1.0, 1.0, 1.0);
            let direction_dot_normal: f32 = dot(&ray_in.direction, &hit_record.normal);
            let (outward_normal, ni_over_nt, cosine) = if direction_dot_normal > 0.0 {
                let rev = Vec3::new(-hit_record.normal.e[0], -hit_record.normal.e[1], -hit_record.normal.e[2]);
                (rev, refractive_idx, refractive_idx * direction_dot_normal / ray_in.direction.length())
            } else {
                (hit_record.normal, 1.0 / refractive_idx, -direction_dot_normal / ray_in.direction.length())
            };

            let (refracted, should_refract) = refract(&ray_in.direction, &outward_normal, ni_over_nt);

            let reflect_prob = if should_refract {
                schlick(cosine, refractive_idx)
            } else {
                1.0
            };

            let mut rng = rand::thread_rng();
            if rng.gen::<f32>() < reflect_prob {
                (attenuation, Ray { origin: hit_record.p, direction: reflected}, true)
            } else {
                (attenuation, Ray { origin: hit_record.p, direction: refracted}, true)
            }
        }
    }
}

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

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v - &(2.0 * dot(v, n) * n)
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> (Vec3, bool) {
    let uv = v.unit_vector();
    let dt = dot(&uv,n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        (&(ni_over_nt * &(&uv - &(n * dt))) - &(n * discriminant.sqrt()), true)
    } else {
        (Vec3::new(0.0, 0.0, 0.0), false)
    }
}

fn schlick(cosine: f32, refractive_idx: f32) -> f32 {
    let mut r0 = (1.0 - refractive_idx) / (1.0 + refractive_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}