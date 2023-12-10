pub mod gaia;
use gaia::*;
use log::{info, trace};

use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;


pub fn generate_catalog(filename: String, threshold: f64){

	let reader = GaiaFileReader::new(filename);
	let stars = reader.read_csv().unwrap();

    //info!("Generating Catalog: {}", &cartesian_points.len());
    //generate_catalog_wip(points, threshold);
    bf_tile_stars(stars);

}


/// A Brute Force Tile Strategy that is no doubt incorrect.
/// 
fn bf_tile_stars(stars: Vec<Star>) { // --> Map<tile_id, Vec<StarPair>> { // idk maybe something like this?

    let fov = 10.0;
    let mut band_counter = 0;
    let mut stars_per_band = 0;
    let mut total_stars = 0;
    for d in (-90..90).step_by(fov as usize){
        let dec_start = d as f64;
        let dec_end = d as f64 + fov;
        let mut ra = 0;
        let mut stars_per_band = 0;
        for r in (0..360).step_by(fov as usize){
            let ra_start = r as f64;
            let ra_end = r as f64 + fov;
            let stars_per_tile  = stars.iter().filter(|f| f.is_in_fov(ra_start, dec_start,  ra_end, dec_end));
            stars_per_band += stars_per_tile.count();
           // println!("{} {} {} {} = count: {}", dec_start, ra_start, dec_end, ra_end, fov_stars.count());
        }
        println!("------------------------------------------------|{band_counter} = ({ra},{d}) - stars in band: {stars_per_band}");
        band_counter += 1;
        total_stars += stars_per_band;
        ra += fov as i32;
    }
    println!("total stars: {} - {}", total_stars, stars.len());

}



fn generate_catalog_wip(stars: Vec<Star>, threshold: f64) {

    let threshold_k = std::f64::consts::PI / threshold;

   // let mut pairs = Vec::new();

    for (i, star) in stars.iter().enumerate() {
        let first = stars[i].clone().to_cartesian();
        let subset: Vec<Point3> =  stars[i+1..].to_vec().into_iter().map(|i| i.to_cartesian()).collect();        

        println!("counter: {:?},   len: {}",i, subset.len());
        let points: Vec<f64> = subset.into_iter().map(|s| s.dot(first).acos()).collect();

        for (j,p) in points.iter().enumerate() {
            if  *p < threshold_k {
                let pair = gaia::StarPair::new(i as i32, j as i32, *p);
             //   pairs.push(pair);
             //   info!("i: {},  j: {},  arcos = {:?}",i, i+1+j, p);
                info!("pair: {:?} - {}",pair, threshold_k);
            }else{
                trace!("P > threshold");
            }            
        }
    }

    println!("\n================================");
    println!("# of stars {}", stars.len());
    //println!("# of pairs {}", pairs.len());
    println!("================================");
    //println!("{:?}",&stars[0] * &stars[1]);
}
