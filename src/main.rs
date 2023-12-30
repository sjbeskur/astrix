
mod cli;
use cli::Config;

fn main() {
	let cfg = Config::parse_args();
	cfg.command.invoke(); // still experimenting with this?
		
	astrix::generate_catalog(cfg.filename, cfg.threshold as f64);

}


