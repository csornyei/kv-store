pub mod commands;
pub mod data;
pub mod session;

use commands::Command;
use data::DataManager;
use session::Session;
use std::{str::FromStr, sync::Arc};

use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

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

        tokio::spawn(handle_client(socket, shared_data));
    }
}

async fn handle_client(mut socket: TcpStream, data: Arc<Mutex<DataManager>>) {
    let mut buf = [0; 1024];
    let mut session = Session::new();

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

                let commands = line.split_inclusive(";");

                for line in commands {
                    if !line.ends_with(";") {
                        session.incomplete_command = line.to_string();
                        continue;
                    }

                    let line = line.trim_end_matches(";");
                    if line.is_empty() {
                        continue;
                    }

                    match Command::from_str(line) {
                        Ok(cmd) => {
                            let result = data.lock().await.handle_command(cmd, session.clone());
                            match result {
                                Ok(response) => {
                                    session = response.1;
                                    let _ = &socket
                                        .write_all(format!("{}\n", response.0).as_bytes())
                                        .await;
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
