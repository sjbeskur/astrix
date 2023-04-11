#[derive(Debug, PartialEq, PartialOrd)]
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
}

