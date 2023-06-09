use std::{ops::Mul};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Point3{
    x: f64,
    y: f64,
    z: f64,
}

impl Point3{
    pub fn new(x: f64, y: f64, z: f64) -> Self{
        Self{x, y, z}
    }

    pub fn x(&self) -> f64{ self.x }

    pub fn y(&self) -> f64{ self.y }

    pub fn z(&self) -> f64{ self.z }

    pub fn dot(&self, other: Point3) -> f64{
        self.x * other.x + self.y * other.y + self.z * other.z
    }

}

impl Mul for Point3{
    type Output = Point3;

    fn mul(self, rhs: Self) -> Self::Output {
        Self{
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }

}

