
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StarPair{
    pub id1: i32,
    pub id2: i32,
    pub angle: f64,
}

impl StarPair {
    pub fn new(id1: i32, id2: i32, angle_radians: f64) -> StarPair {
        Self{
            id1, id2, angle: angle_radians,
        }
    }
}

