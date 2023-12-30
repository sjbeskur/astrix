#![allow(dead_code)]

use clap::{Parser,Subcommand,Args};
use dotenv::dotenv;
use tracing_log::AsTrace;

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

	#[clap(subcommand)]
	pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command{
	InitDB{
		/// Database connection string
		#[arg(short, long, env("DATABASE_URL"))]
		database_url: String,		
	}
}

impl Config {
	pub fn parse_args() -> Self {
		dotenv().ok(); // reads from the .env file 
		Config::parse().config_logger()
	}

	/// Examples here:
	/// https://github.com/clap-rs/clap-verbosity-flag/blob/master/examples/tracing.rs
	fn config_logger(self) -> Self{ //  config: &Config
		let filter = self.verbose.log_level_filter().as_trace();
		tracing_subscriber::fmt()
			.with_max_level(filter)
			.init();
		self
	}	
}

impl Command{

	pub fn invoke(&self){
		match &self {
			Self::InitDB { database_url } =>{
				println!("url is {:?}", database_url);
			}
		}
	}
}