use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub},
};

use image::Rgb;

#[derive(Clone, Copy, Debug)]
pub struct Vector3([f64; 3]);

pub type Color = Vector3;
pub type Point3 = Vector3;

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3([x, y, z])
    }

    pub fn x(&self) -> f64 {
        self[0]
    }

    pub fn y(&self) -> f64 {
        self[1]
    }

    pub fn z(&self) -> f64 {
        self[2]
    }

    pub fn r(&self) -> u8 {
        (self[0] * 256.0).clamp(0.0, 255.0) as u8
    }

    pub fn g(&self) -> u8 {
        (self[1] * 256.0).clamp(0.0, 255.0) as u8
    }

    pub fn b(&self) -> u8 {
        (self[2] * 256.0).clamp(0.0, 255.0) as u8
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    pub fn dot(self, rhs: Vector3) -> f64 {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2]
    }

    pub fn cross(self, rhs: Vector3) -> Vector3 {
        Self([
            self[1] * rhs[2] - self[2] * rhs[1],
            self[2] * rhs[0] - self[0] * rhs[2],
            self[0] * rhs[1] - self[1] * rhs[0],
        ])
    }

    pub fn unit_vector(self) -> Vector3 {
        self / self.length()
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Self::Output {
        Self([self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]])
    }
}

impl AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Self::Output {
        Self([self[0] / rhs, self[1] / rhs, self[2] / rhs])
    }
}

impl DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, rhs: f64) {
        self[0] /= rhs;
        self[1] /= rhs;
        self[2] /= rhs;
    }
}

impl Index<usize> for Vector3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Self([self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]])
    }
}

impl Mul<u32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: u32) -> Self::Output {
        Self([
            self[0] * rhs as f64,
            self[1] * rhs as f64,
            self[2] * rhs as f64,
        ])
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self([self[0] * rhs, self[1] * rhs, self[2] * rhs])
    }
}

impl Mul<Vector3> for u32 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3([
            rhs[0] * self as f64,
            rhs[1] * self as f64,
            rhs[2] * self as f64,
        ])
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3([rhs[0] * self, rhs[1] * self, rhs[2] * self])
    }
}

impl MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, rhs: f64) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Self([-self[0], -self[1], -self[2]])
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Self::Output {
        Self([self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]])
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}, {}, {}>", self[0], self[1], self[2])
    }
}

impl From<Vector3> for Rgb<u8> {
    fn from(value: Vector3) -> Self {
        Rgb([value.r(), value.g(), value.b()])
    }
}
