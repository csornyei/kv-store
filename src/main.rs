use kvstore::start_server;
use kvstore::{data::DataManager, persistence::Persistence};
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

    // Persistence type
    #[clap(short, long, default_value = "in_memory")]
    persistence: String,

    // JSON file path to persist data
    #[clap(short, long, default_value = "")]
    file_path: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let persistence = match args.persistence.as_str() {
        "in_memory" => Persistence::new_in_memory(),
        "json" => Persistence::new_json_file(args.file_path),
        _ => return Err("Invalid persistence type!".into()),
    };

    let data_manager = DataManager::new(args.username, args.password, persistence)
        .expect("Failed to create data manager");

    let data = Arc::new(Mutex::new(data_manager));

    start_server(&args.address, args.port, data).await?;

    Ok(())
}
