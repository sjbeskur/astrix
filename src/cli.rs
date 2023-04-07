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
}
