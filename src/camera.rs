use crate::vec::{Vec3, Point3, cross};
use crate::ray::Ray;
use std::f32::consts::PI;

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
}

impl Camera {
    pub fn new(lookfrom: Point3, lookat: Point3, vup: Vec3, vof: f32, aspect_ratio: f32) -> Camera {
        let theta = degrees_to_radians(vof);
        let h = (theta / 2.0).tan();
        let viewport_height = h * 2.0;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::unit_vector(&(lookfrom - lookat));
        let u = Vec3::unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);

        let _o = lookfrom;
        let _h = viewport_width * u;
        let _v = viewport_height * v;
        let _llc = _o - _h/2.0 - _v/2.0 - w;

        Camera {
            origin: _o,
            horizontal: _h,
            vertical: _v,
            lower_left_corner: _llc
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin
        }
    }
}