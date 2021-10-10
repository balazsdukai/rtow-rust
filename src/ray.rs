use crate::vec::{Vec3, Point3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            origin,
            direction,
        }
    }

    pub fn point_at_parameter(&self, t: f32) -> Point3 {
        &self.origin + &((&self.direction) * t)
    }
}