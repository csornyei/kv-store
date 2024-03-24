use kvstore::config::Config;
use kvstore::start_server;
use std::error::Error;

use clap::Parser;

// Key-Value Store Server
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // JSON file path to persist data
    #[clap(short = 'c', long = "config")]
    config_path: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let config = Config::load(args.config_path);

    start_server(config).await?;

    Ok(())
}
