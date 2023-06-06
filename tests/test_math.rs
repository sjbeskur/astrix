use nalgebra::*;

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