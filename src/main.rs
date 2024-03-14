use kvstore::commands::Command;
use kvstore::utils::ThreadPool;
use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let thread_pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread_pool.execute(|| {
            handle_client(stream);
        });
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {
                let line = String::from_utf8_lossy(&buffer);
                let line = line.trim_matches('\0');
                match Command::from_string(line.to_string()) {
                    Ok(cmd) => {
                        println!("Command: {:?}", cmd.name);
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
