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
    vof: f32,
    aspect_ratio: f32
}

impl Camera {
    pub fn new(vof: f32, aspect_ratio: f32) -> Camera {
        Camera {
            lower_left_corner: Point3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
            origin: Point3::new(0.0, 0.0, 0.0),
            vof: vof,
            aspect_ratio: aspect_ratio
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: &self.lower_left_corner + &(&(u*&self.horizontal) + &(v*&self.vertical))
        }
    }
}