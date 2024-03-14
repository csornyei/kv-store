use kvstore::commands::Command;
use kvstore::data::DataManager;
use kvstore::utils::ThreadPool;
use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    str::FromStr,
    sync::{Arc, Mutex},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let thread_pool = ThreadPool::new(4);

    let data = Arc::new(Mutex::new(DataManager::new()));

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let shared_data = Arc::clone(&data);

        thread_pool.execute(move || {
            handle_client(stream, shared_data);
        });
    }
}

fn handle_client(mut stream: TcpStream, data: Arc<Mutex<DataManager>>) {
    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(size) => {
                let line = String::from_utf8_lossy(&buffer[..size]);
                let line = line.trim();
                match Command::from_str(line) {
                    Ok(cmd) => {
                        let result = data.lock().unwrap().handle_command(cmd);
                        match result {
                            Ok(response) => {
                                let _ = &stream.write_all(response.as_bytes()).unwrap();
                                let _ = &stream.write_all(b"\n").unwrap();
                            }
                            Err(e) => {
                                let _ = &stream.write_all(e.as_bytes()).unwrap();
                                let _ = &stream.write_all(b"\n").unwrap();
                            }
                        }
                    }
                    Err(e) => {
                        let _ = &stream.write_all(e.to_string().as_bytes()).unwrap();
                        let _ = &stream.write_all(b"\n").unwrap();
                    }
                }
            }
            Err(e) => {
                println!("Failed to read from connection: {}", e);
                break;
            }
        }
    }
}
