use clap::Parser;
use tracing_log::AsTrace;

mod cli;
pub use cli::{Config};

fn main() {
	let cfg = Config::parse();
	config_logger(&cfg);
		
	astrix::generate_catalog(cfg.filename, cfg.threshold as f64);

}

/// Examples here:
/// https://github.com/clap-rs/clap-verbosity-flag/blob/master/examples/tracing.rs
fn config_logger(config: &Config){
	let filter = config.verbose.log_level_filter().as_trace();
	tracing_subscriber::fmt()
		.with_max_level(filter)
		.init();
}
