pub mod auth;
pub mod commands;
pub mod data;
pub mod handler;
pub mod persistence;
pub mod session;

use data::DataManager;
use handler::ClientHandler;
use std::sync::Arc;

use tokio::net::TcpListener;
use tokio::sync::Mutex;

pub async fn start_server(
    address: &str,
    port: u16,
    data: Arc<Mutex<DataManager>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(format!("{}:{}", address, port)).await?;
    println!("Server listening on port {}", port);

    loop {
        let (socket, _) = listener.accept().await?;

        println!("Accepted connection from: {}", socket.peer_addr()?);

        let shared_data = Arc::clone(&data);

        let client_handler = ClientHandler::new(socket, shared_data);

        client_handler.spawn_handler().await;
    }
}
