use kvstore::commands::Command;
use kvstore::data::DataManager;
use std::{
    str::FromStr,
    sync::{Arc, Mutex},
};

use tokio::net::TcpListener;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = Arc::new(Mutex::new(DataManager::new()));

    start_server("127.0.0.1", 8080, data).await?;

    Ok(())
}

async fn start_server(
    address: &str,
    port: u16,
    data: Arc<Mutex<DataManager>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(format!("{}:{}", address, port)).await?;
    println!("Server listening on port 8080");

    loop {
        let (socket, _) = listener.accept().await?;

        println!("Accepted connection from: {}", socket.peer_addr()?);

        let shared_data = Arc::clone(&data);

        tokio::spawn(handle_client(socket, shared_data));
    }
}

async fn handle_client(mut socket: TcpStream, data: Arc<Mutex<DataManager>>) {
    let mut buf = [0; 1024];

    loop {
        match socket.read(&mut buf).await {
            Ok(0) => return,
            Err(e) => {
                eprintln!("failed to read from socket; err = {:?}", e);
                return;
            }
            Ok(n) => {
                let line = String::from_utf8_lossy(&buf[..n]);
                let line = line.trim();
                match Command::from_str(line) {
                    Ok(cmd) => {
                        let result = data.lock().unwrap().handle_command(cmd);
                        match result {
                            Ok(response) => {
                                let _ =
                                    &socket.write_all(format!("{}\n", response).as_bytes()).await;
                            }
                            Err(e) => {
                                let _ = &socket.write_all(format!("{}\n", e).as_bytes()).await;
                            }
                        }
                    }
                    Err(e) => {
                        let _ = &socket.write_all(e.to_string().as_bytes()).await;
                        let _ = &socket.write_all(b"\n").await;
                    }
                }
            }
        }
    }
}

}
