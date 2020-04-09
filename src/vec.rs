use std::ops::{Add, AddAssign, Sub, Mul, Div};


// Vec3
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub e: [f32; 3]
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Vec3 {
        Vec3 {
            e: [e0, e1, e2]
        }
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn r(&self) -> f32 {
        self.e[0]
    }

    pub fn g(&self) -> f32 {
        self.e[1]
    }

    pub fn b(&self) -> f32 {
        self.e[2]
    }

    pub fn length(&self) -> f32 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }

    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }

}

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Self::Output {
        Vec3::new(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2]
        )
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self::Output {
        Vec3::new(
            self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2]
        )
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2]
            ]
        };
    }
}

impl Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 { e: [
            self.e[0] * rhs,
            self.e[1] * rhs,
            self.e[2] * rhs
        ] }
    }
}

impl Mul<&Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 { e: [
            self * rhs.e[0],
            self * rhs.e[1],
            self * rhs.e[2]
        ] }
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3 { e: [
            self.e[0] / rhs.e[0],
            self.e[1] / rhs.e[1],
            self.e[2] / rhs.e[2]
        ] }
    }
}

impl Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3 { e: [
            self.e[0] / rhs,
            self.e[1] / rhs,
            self.e[2] / rhs
        ] }
    }
}

pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
    (v1.e[0] * v2.e[0] + v1.e[1] * v2.e[1] + v1.e[2] * v2.e[2])
}