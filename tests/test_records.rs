use astrix::gaia::*;

#[test]
fn test_angular_separation() {
    let star1 = GaiaRecord::new(101.287024, -16.716001, -1.5);
    let star2 = GaiaRecord::new(95.987955, -52.695661, -0.62);

    let angle = star1.angular_separation(&star2);
    println!("Angular separation: {}", angle);
}


#[test]
fn test_angular_separation_of_big_dipper_stars() {
    let alkaid = GaiaRecord::new(206.9875, 49.313, 1.5);
    let mizar = GaiaRecord::new(199.4925, 54.925, 1.5);
    
    let alioth = GaiaRecord::new(192.7533, 55.959, 1.5);
    let megrez = GaiaRecord::new(181.5296, 57.017, 1.5);
    let phecda = GaiaRecord::new(179.7404, 53.694, 1.5);
    let merak = GaiaRecord::new(165.1371, 56.382, 1.5);
    let dubhe = GaiaRecord::new(165.2625, 61.750, 1.5);

    let angle = alkaid.angular_separation(&mizar);
    println!("Angular separation: {}", angle);

    let angle = mizar.angular_separation(&alioth);
    println!("Angular separation: {}", angle);
    let angle = alioth.angular_separation(&megrez);
    println!("Angular separation: {}", angle);
}
