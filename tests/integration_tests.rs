extern crate kvstore;

use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use kvstore::data::DataManager;
use kvstore::start_server;

const ADDRESS: &str = "127.0.0.1";

async fn start_test_server(port: u16) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let data = Arc::new(Mutex::new(
            DataManager::new("admin".to_string(), "Password4".to_string())
                .expect("Failed to create data manager!"),
        ));
        start_server(ADDRESS, port, data).await.unwrap();
    })
}

async fn send_command(mut client: TcpStream, command: &str) -> (TcpStream, String) {
    let _ = client.write_all(command.as_bytes()).await;
    let mut buf = [0; 1024];

    let n = client.read(&mut buf).await.unwrap();

    (client, String::from_utf8_lossy(&buf[..n]).to_string())
}

#[tokio::test]
async fn test_simple_key_value_flow() {
    const PORT: u16 = 65500;

    let server_handle = start_test_server(PORT).await;

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let client = TcpStream::connect(format!("{}:{}", ADDRESS, PORT))
        .await
        .unwrap();

    let (client, response) = send_command(client, "AUTH admin Password4\n").await;
    assert_eq!(response, "OK\n");

    let (client, response) = send_command(client, "SET key value\n").await;
    assert_eq!(response, "OK\n");

    let (client, response) = send_command(client, "GET key\n").await;
    assert_eq!(response, "value\n");

    let (client, response) = send_command(client, "DEL key\n").await;
    assert_eq!(response, "OK\n");

    let (_, response) = send_command(client, "GET key\n").await;
    assert_eq!(response, "Key not found\n");

    server_handle.abort();
}

#[tokio::test]
async fn test_two_clients() {
    const PORT: u16 = 65501;
    let server_handle = start_test_server(PORT).await;

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let first_client = TcpStream::connect(format!("{}:{}", ADDRESS, PORT))
        .await
        .unwrap();

    let (first_client, response) = send_command(first_client, "AUTH admin Password4\n").await;
    assert_eq!("OK\n", response);

    let second_client = TcpStream::connect(format!("{}:{}", ADDRESS, PORT))
        .await
        .unwrap();

    let (second_client, response) = send_command(second_client, "AUTH admin Password4\n").await;
    assert_eq!(response, "OK\n");

    let (first_client, response) = send_command(first_client, "SET key value\n").await;
    assert_eq!(response, "OK\n");

    let (second_client, response) = send_command(second_client, "GET key\n").await;
    assert_eq!(response, "value\n");

    let (second_client, response) = send_command(second_client, "SET key value2\n").await;
    assert_eq!(response, "OK\n");

    let (first_client, response) = send_command(first_client, "GET key\n").await;
    assert_eq!(response, "value2\n");

    let (first_client, response) = send_command(first_client, "DEL key\n").await;
    assert_eq!(response, "OK\n");

    let (_, response) = send_command(first_client, "GET key\n").await;
    assert_eq!(response, "Key not found\n");

    let (_, response) = send_command(second_client, "GET key\n").await;
    assert_eq!(response, "Key not found\n");

    server_handle.abort();
}

#[tokio::test]
async fn test_lot_clients() {
    const PORT: u16 = 65502;
    let server_handle = start_test_server(PORT).await;

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let mut handles = Vec::new();

    for i in 0..10 {
        let client = TcpStream::connect(format!("{}:{}", ADDRESS, PORT))
            .await
            .unwrap();

        let handle = tokio::spawn(async move {
            let (client, response) = send_command(client, "AUTH admin Password4\n").await;
            assert_eq!(response, "OK\n");

            let (client, response) = send_command(client, &format!("SET key{} value\n", i)).await;
            assert_eq!(response, "OK\n", "SET command response mismatch");

            let (client, response) = send_command(client, &format!("GET key{}\n", i)).await;
            assert_eq!(
                response, "value\n",
                "GET command (after SET) response mismatch"
            );

            let (client, response) = send_command(client, &format!("DEL key{}\n", i)).await;
            assert_eq!(response, "OK\n", "DEL command response mismatch");

            let (_, response) = send_command(client, &format!("GET key{}\n", i)).await;
            assert_eq!(
                response, "Key not found\n",
                "GET command (after DEL) response mismatch"
            );
        });

        handles.push(handle);
    }

    let results = futures::future::join_all(handles).await;
    for result in results {
        assert!(
            result.is_ok(),
            "A task has panicked or failed its assertions"
        );
    }

    server_handle.abort();
}
