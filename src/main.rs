use kvstore::data::DataManager;
use kvstore::start_server;
use std::{error::Error, sync::Arc};
use tokio::sync::Mutex;

use clap::Parser;

// Key-Value Store Server
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Name for the default user
    #[clap(short, long, default_value = "admin")]
    username: String,

    // Password for the default user
    #[clap(short = 'P', long, default_value = "Admin1234")]
    password: String,

    // Address to bind the server to
    #[clap(short, long, default_value = "127.0.0.1")]
    address: String,

    // Port to bind the server to
    #[clap(short, long, default_value = "8080")]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let data_manager =
        DataManager::new(args.username, args.password).expect("Failed to create data manager!");

    let data = Arc::new(Mutex::new(data_manager));

    start_server(&args.address, args.port, data).await?;

    Ok(())
}
