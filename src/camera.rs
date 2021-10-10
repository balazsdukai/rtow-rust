use crate::vec::{Vec3, Point3};
use crate::ray::Ray;

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            lower_left_corner: Point3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
            origin: Point3::new(0.0, 0.0, 0.0)
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: &self.lower_left_corner + &(&(u*&self.horizontal) + &(v*&self.vertical))
        }
    }
}