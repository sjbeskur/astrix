use tracing_subscriber::reload::Handle;


#[derive(Debug, Copy, Clone)]
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


impl std::hash::Hash for StarPair{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let hash = (self.id2 * self.id1) + (self.id1 + self.id2); // this should be unique and independent of order according to some dude :D
        hash.hash(state);
    }
}

impl std::cmp::Eq for StarPair{}

impl std::cmp::PartialEq for StarPair{
    fn eq(&self, other: &Self) -> bool {
        self.id1 == other.id1 && self.id2 == other.id2 || 
        self.id1 == other.id2 && self.id2 == other.id1 
    }
}

#[test]
fn hash_star_pair_test() { 
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let pair1 = StarPair::new(2,312, 0.0);
    let pair2 = StarPair::new(312,2, 0.0);

    // let mut hasher = DefaultHasher::new();
    // pair1.hash(&mut hasher);    
    // println!("Hash is {:x}!", hasher.finish());

    // pair2.hash(&mut hasher);    
    // println!("Hash is {:x}!", hasher.finish());

    let mut map = std::collections::HashSet::new();
    map.insert(pair1);
    assert_eq!( map.contains(&pair2), true);
    assert_eq!( map.contains(&pair1), true);


}