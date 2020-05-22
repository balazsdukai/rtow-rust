use crate::vec::{Vec3, dot};
use crate::ray::Ray;
use crate::material::Material;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub on_edge: bool,
    pub material: Material,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            t: 0.0,
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            on_edge: false,
            material: Material::Lambertian {albedo: Vec3::new(0.0,0.0,0.0)}
        }
    }
}

pub trait Hitable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material
}

impl Hitable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc: Vec3 = &r.origin - &self.center; // origin coordinate
        let a: f32 = dot(&r.direction, &r.direction);
        let b: f32 = 2.0 * dot(&oc, &r.direction);
        let c: f32 = dot(&oc, &oc) - &self.radius * &self.radius;
        let discriminant: f32 = b*b - 4_f32*a*c;
        if discriminant > 0.0 {
            let mut hitpoint = (-b - discriminant.sqrt()) / (2.0 * a);
            if hitpoint < t_max && hitpoint > t_min {
                let t: f32 = hitpoint;
                let p: Vec3 = r.point_at_parameter(t);
                let normal: Vec3 = &(&p - &self.center) / self.radius;
                if discriminant < 0.0005 {
                    return Some(HitRecord{t, p, normal, on_edge: true, material: self.material.clone()});
                }
                else {
                    return Some(HitRecord{t, p, normal, on_edge: false, material: self.material.clone()});
                }
            }
            hitpoint = (-b + discriminant.sqrt()) / (2.0 * a);
            if hitpoint < t_max && hitpoint > t_min {
                let t: f32 = hitpoint;
                let p: Vec3 = r.point_at_parameter(t);
                let normal: Vec3 = &(&p - &self.center) / self.radius;
                if discriminant < 0.0005 {
                    return Some(HitRecord{t, p, normal, on_edge: true, material: self.material.clone()});
                }
                else {
                    return Some(HitRecord{t, p, normal, on_edge: false, material: self.material.clone()});
                }
            }
        }
        None
    }
}

pub struct HitableList {
    pub list: Vec<Box<dyn Hitable>>
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut temp_rec: Option<HitRecord> = None;

        for hitable in &self.list {
            if let Some(hit_record) = hitable.hit(r, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                temp_rec = Some(hit_record)
            }
        }
        temp_rec
    }
}