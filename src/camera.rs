use crate::vec::{Vec3, Point3};
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
    pub fn new(vof: f32, aspect_ratio: f32) -> Camera {
        let theta = degrees_to_radians(vof);
        let h = (theta / 2.0).tan();
        let viewport_height = h * 2.0;
        let viewport_width = aspect_ratio * viewport_height;

        let focal_length = 1.0;

        let _o = Point3::new(0.0, 0.0, 0.0);
        let _h = Vec3::new(viewport_width, 0.0, 0.0);
        let _v = Vec3::new(0.0, viewport_height, 0.0);
        let _llc = _o - _h/2.0 - _v/2.0 - Vec3::new(0.0, 0.0, focal_length);

        Camera {
            origin: _o,
            horizontal: _h,
            vertical: _v,
            lower_left_corner: _llc
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin
        }
    }
}