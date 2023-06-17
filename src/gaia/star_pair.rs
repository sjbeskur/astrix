
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

#[test]
pub fn test_star_pair_new() {
    let pair = StarPair::new(55,45,60.0_f64.to_radians());
    assert!(pair.id1 == 55);
    assert!(pair.id2 == 45);
    assert!(round(pair.angle.cos(), 3) == 0.5);
}

fn round(x: f64, decimals: u32) -> f64 {
    let y = 10i64.pow(decimals) as f64;
    (x * y).round() / y
}
