
mod cli;
use cli::Config;

#[tokio::main]
async fn main() {
	let cfg = Config::parse_args();
	cfg.command.invoke(cfg.filename).await; // still experimenting with this?
		
	//astrix::generate_catalog(cfg.filename, cfg.threshold as f64);

}


