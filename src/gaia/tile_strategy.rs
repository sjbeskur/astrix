use super::Star;
use log::{info, trace};

pub trait TileStrategy{
    fn build_tiles(&self, stars: Vec<Star>);
}


pub struct BFTileStrategy;

/// A Brute Force Tile Strategy that is no doubt incorrect.
/// s
impl TileStrategy for BFTileStrategy {
    fn build_tiles(&self, stars: Vec<Star>) { // --> Map<tile_id, Vec<StarPair>> { // idk maybe something like this?

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
}

