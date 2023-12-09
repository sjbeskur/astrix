use clap::Parser;

#[derive(Debug, Parser)]
#[command(
	author = "Sam Beskur <sam.beskur@gmail.com>",
	version,
	about = "Astrix",
	long_about = "Another STar RecogonItion eXperiment."
)]
pub struct Config {
	pub filename: String,
	
	#[arg(short = 't', long = "threshold", default_value_t = 4)]
	pub threshold: u32,

	#[clap(flatten)]
	pub verbose: clap_verbosity_flag::Verbosity,
}

impl Config {
	pub fn parse_args() -> Self {
		Config::parse()
	}
}