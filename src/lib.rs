pub mod gaia;
use gaia::*;

use std::sync::mpsc::channel;
use std::thread;


pub fn generate_catalog(filename: String, threshold: f64){

	let (tx, rx) = channel();

	let handle = thread::spawn(move || {
		let mut prev: Option<Star> = None;
		while let Ok(result) = rx.recv() {
			let curr = result;
			if let Some(prev) = prev {
				let angle = prev.angular_separation(&curr);
				let cart =  prev.to_cartesian();
				//println!("angle: {}   cart: {:?}", angle, cart);
			}
			prev = Some(curr);
		}
	});

	let reader = GaiaFileReader::new(filename);
	let _ = reader.read_csv(tx);

	handle.join().unwrap();

}