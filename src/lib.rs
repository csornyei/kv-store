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

fn parse_line(buf: [u8; 1024], line_length: usize) -> String {
    let line = String::from_utf8_lossy(&buf[..line_length]);
    let line = line.trim();
    let line = String::from(line);
    line
}

fn split_line(line: String) -> Vec<String> {
    line.split_inclusive(";")
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
}

fn continue_command(session: &mut Session, mut commands: Vec<String>) -> Vec<String> {
    if session.incomplete_command.len() > 0 {
        if commands.len() > 0 {
            let first_command = commands.remove(0);
            let real_first_command = format!("{} {}", session.incomplete_command, first_command);
            commands.insert(0, real_first_command);
            session.incomplete_command = "".to_string();
        }
    }
    commands
}

async fn execute_command(
    data: Arc<Mutex<DataManager>>,
    session: Session,
    command: Command,
) -> Result<(String, Session), String> {
    let mut data = data.lock().await;
    data.handle_command(command, session)
}

fn handle_command_result(
    result: Result<(String, Session), String>,
    session: &mut Session,
) -> String {
    match result {
        Ok((response, new_session)) => {
            session.update(new_session);
            response.to_string()
        }
        Err(e) => e.to_string(),
    }
}

async fn write_results(mut socket: TcpStream, results: Vec<String>) -> TcpStream {
    let mut results_string = results.join(";");

    if !results_string.ends_with(";") && results_string.len() > 0 {
        results_string = results_string + ";";
    }

    if results_string.len() == 0 {
        results_string = " ".to_string();
    }

    let _ = socket.write_all(results_string.as_bytes()).await;

    socket
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
                let line = parse_line(buf, n);

                let commands = split_line(line);

                let mut commands = continue_command(&mut session, commands);

                if !commands[commands.len() - 1].ends_with(";") {
                    session.incomplete_command = commands[commands.len() - 1].to_string();
                    commands.pop();
                }

                let commands = commands
                    .iter()
                    .map(|line| line.trim_end_matches(";"))
                    .collect::<Vec<&str>>();

                let mut results = Vec::new();

                for line in commands {
                    match Command::from_str(line) {
                        Ok(cmd) => {
                            results.push(handle_command_result(
                                execute_command(Arc::clone(&data), session.clone(), cmd).await,
                                &mut session,
                            ));
                        }
                        Err(e) => {
                            results.push(e.to_string());
                        }
                    }
                }

                socket = write_results(socket, results).await;
            }
        }
    }
}
