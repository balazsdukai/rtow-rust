use crate::vec::{Vec3, Point3, cross, random_in_unit_disk};
use crate::ray::Ray;
use std::f32::consts::PI;
use rand::prelude::ThreadRng;

fn degrees_to_radians(rad: f32) -> f32 {
    rad / 180.0 * PI
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn degree_convert () {
        assert_ne!(1.57, degrees_to_radians(90.0));
    }
}

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vof: f32,
        aspect_ratio: f32,
        aperture: f32,
        focust_dist: f32
    ) -> Camera {
        let theta = degrees_to_radians(vof);
        let h = (theta / 2.0).tan();
        let viewport_height = h * 2.0;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::unit_vector(&(lookfrom - lookat));
        let u = Vec3::unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);

        let origin = lookfrom;
        let horizontal = focust_dist * viewport_width * u;
        let vertical = focust_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focust_dist*w;

        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            lens_radius
        }
    }

    pub fn get_ray(&self, s: f32, t: f32, rng: &mut ThreadRng) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk(rng);
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        }
    }
}