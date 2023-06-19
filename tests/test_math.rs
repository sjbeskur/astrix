use nalgebra::*;

use astrix::*;
use astrix::gaia::*;

#[test]
fn test_angle_calculation() {

    //let v1 = vector![i as f64 * 0.5,  0.5, 0.0];
    //let v2 = vector![ i as f64 * -0.5, 0.5, 0.0];
    let v1 = vector![1.0,  2.0, 3.0];
    let v2 = vector![-2.0, 1.0, 5.0];
    
    let dot = v1.dot(&v2);
    
    println!("{} .dot({})",v1, v2);    
    println!("dot = {}",dot);
    println!("dot.acos() = {}",dot.acos());

    let mags = v1.norm() * v2.norm();
    let angle = (dot / mags as f64).acos();

    println!("angle = {}",angle);

    assert!(dot.acos() == angle);
}


#[test]
fn test_list_calcs() {

    let stars = vec![
        astrix::gaia::Star::new(101.287024,-16.716001,-1.5).to_cartesian(),
        astrix::gaia::Star::new(95.987955,-52.695661,-0.62).to_cartesian(),
        astrix::gaia::Star::new(219.899787,-60.835273,0.0).to_cartesian(),
        astrix::gaia::Star::new(213.915319,19.18245,0.0).to_cartesian(),
        astrix::gaia::Star::new(119.836786,84.50168,11.99).to_cartesian(),
        astrix::gaia::Star::new(113.833629,72.756157,11.99).to_cartesian(),
        astrix::gaia::Star::new(175.670514,72.817919,11.99).to_cartesian(),
        astrix::gaia::Star::new(97.873306,67.499984,11.99).to_cartesian(),
        astrix::gaia::Star::new(229.875702,67.504577,11.99).to_cartesian(),
        astrix::gaia::Star::new(276.891453,65.282757,11.99).to_cartesian(),
        astrix::gaia::Star::new(321.697102,85.237741,11.99).to_cartesian(),
        astrix::gaia::Star::new(63.498991,73.194722,11.99).to_cartesian(),
        astrix::gaia::Star::new(261.47561,73.264174,11.99).to_cartesian()
    ];

    let threshold_k = std::f64::consts::PI / 4.0;

    let mut pairs = Vec::new();

    for (i, star) in stars.iter().enumerate() {
        let mut first = stars[i].clone();
        let subset: Vec<gaia::Point3> =  stars[i+1..].to_vec().into_iter().map(|i| i).collect();        

        println!("counter: {:?},   len: {}",i, subset.len());

        let points: Vec<f64> = subset.into_iter().map(|s| s.dot(first).acos()).collect();

        for (j,p) in points.iter().enumerate() {
            if  *p < threshold_k {
                pairs.push(gaia::StarPair::new(i as i32, j as i32, *p));
                println!("i: {},  j: {},  arcos = {:?}",i, i+1+j, p);
            }else{
                println!("P > threshold");
            }            
        }

    }

    println!("\n================================");
    println!("# of stars {}", stars.len());
    println!("# of pairs {}", pairs.len());
    println!("================================");

    //println!("{:?}",&stars[0] * &stars[1]);

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
