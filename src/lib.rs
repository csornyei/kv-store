pub mod auth;
pub mod commands;
pub mod config;
pub mod data;
pub mod handler;
pub mod persistence;
pub mod session;

use config::Config;
use data::Store;
use handler::ClientHandler;
use persistence::PersistenceType;
use std::sync::Arc;

use tokio::net::TcpListener;
use tokio::sync::Mutex;

pub async fn start_server(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let store = if config.persistence.get_type() == PersistenceType::JsonFile {
        config.persistence.load_store()?
    } else {
        Store::new(".".to_string())
    };

    let data = Arc::new(Mutex::new(store));

    let listener = TcpListener::bind(config.get_server_address()).await?;
    println!("Key-Value Server is listening");

    let config = Arc::new(Mutex::new(config));

    loop {
        let (socket, _) = listener.accept().await?;

        println!("Accepted connection from: {}", socket.peer_addr()?);

        let shared_data = Arc::clone(&data);

        let shared_config = Arc::clone(&config);

        let client_handler = ClientHandler::new(socket, shared_data, shared_config);

        client_handler.spawn_handler().await;
    }
}
