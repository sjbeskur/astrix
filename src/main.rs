use astrix::gaia::*;
use clap::Parser;
use std::sync::mpsc::channel;

mod cli;
fn main() {
	let args = cli::Config::parse();
	let file_name = args.filename;

	let (tx, rx) = channel();

	let handle = std::thread::spawn(move || {
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

	let reader = GaiaFileReader::new(file_name);
	let _ = reader.read_csv(tx);

	handle.join().unwrap();
}
