use astrix::gaia::*;
use clap::Parser;
use tracing_log::AsTrace;
use std::sync::mpsc::channel;
use std::thread;
mod cli;

fn main() {
	let args = cli::Config::parse();
	config_logger(&args);
	
	let file_name = args.filename;

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

	let reader = GaiaFileReader::new(file_name);
	let _ = reader.read_csv(tx);

	handle.join().unwrap();
}

fn config_logger(config: &cli::Config){
	let filter = config.verbose.log_level_filter().as_trace();
	tracing_subscriber::fmt()
		.with_max_level(filter)
		.init();
}
