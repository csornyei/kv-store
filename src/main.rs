use kvstore::persistence::PersistenceType;
use kvstore::start_server;
use kvstore::{config::Config, data::Store};
use std::{error::Error, sync::Arc};
use tokio::sync::Mutex;

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

    let store = if config.persistence.get_type() == PersistenceType::JsonFile {
        config.persistence.load_store()?
    } else {
        Store::from_config(&config)
    };

    let data = Arc::new(Mutex::new(store));

    start_server(&config.server.address, config.server.port, data).await?;

    Ok(())
}
