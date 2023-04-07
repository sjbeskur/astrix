use astrix::gaia::*;

#[test]
fn test_angular_separation() {
    let star1 = Star::new(101.287024, -16.716001, -1.5);
    let star2 = Star::new(95.987955, -52.695661, -0.62);

    let angle = star1.angular_separation(&star2);
    println!("Angular separation: {}", angle);
}

#[test]
fn test_angular_separation_of_big_dipper_stars() {
    let alkaid = Star::new(206.8846, 49.3131, 1.5);
    let mizar = Star::new(200.9812, 54.9253, 1.5);
    let alioth = Star::new(192.7533, 55.959, 1.5);
    let megrez = Star::new(181.5296, 57.017, 1.5);
    let phecda = Star::new(179.7404, 53.694, 1.5);
    let merak = Star::new(165.1371, 56.382, 1.5);
    let dubhe = Star::new(165.9321, 61.7508, 1.5);

    println!("Alkaid - Mizar : {} ", alkaid.angular_separation(&mizar));
    println!("Mizar  - Alioth: {} ", mizar.angular_separation(&alioth));
    println!("Alioth - Megrez: {} ", alioth.angular_separation(&megrez));
    println!("Megrez - Phecda: {} ", megrez.angular_separation(&phecda));
    println!("Phecda - Merak : {} ", phecda.angular_separation(&merak));

    println!("Merak - Alkaid : {} ", merak.angular_separation(&alkaid));
    println!("Dubhe - Alkaid : {} ", dubhe.angular_separation(&alkaid));
}
