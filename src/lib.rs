pub mod gaia;
use gaia::*;
use gaia::tile_strategy::*;

use log::{info, trace};

use std::sync::mpsc::channel;
use std::sync::mpsc::{Sender, Receiver};
use std::thread;


pub fn generate_catalog(filename: String, threshold: f64){

	let reader = GaiaFileReader::new(filename);
	let stars = reader.read_csv().unwrap();

    //info!("Generating Catalog: {}", &cartesian_points.len());
    //generate_catalog_wip(points, threshold);
    build_tiles::<BFTileStrategy>(BFTileStrategy, stars);

}


fn build_tiles<T: TileStrategy>(strategy: T ,stars: Vec<Star>) { 
    strategy.build_tiles(stars);
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
