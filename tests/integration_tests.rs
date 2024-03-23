extern crate kvstore;

use kvstore::persistence::Persistence;
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
            DataManager::new(
                "admin".to_string(),
                "Password4".to_string(),
                Persistence::new_in_memory(),
            )
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
async fn test_integration_simple_key_value_flow() {
    const PORT: u16 = 65500;

    let server_handle = start_test_server(PORT).await;

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let client = TcpStream::connect(format!("{}:{}", ADDRESS, PORT))
        .await
        .unwrap();

    let (client, response) = send_command(client, "AUTH admin Password4;").await;
    assert_eq!(response, "OK;");

    let (client, response) = send_command(client, "SET key value;").await;
    assert_eq!(response, "OK;");

    let (client, response) = send_command(client, "GET key;").await;
    assert_eq!(response, "value;");

    let (client, response) = send_command(client, "DEL key;").await;
    assert_eq!(response, "OK;");

    let (_, response) = send_command(client, "GET key;").await;
    assert_eq!(response, "Key not found;");

    server_handle.abort();
}

#[tokio::test]
async fn test_integration_two_clients() {
    const PORT: u16 = 65501;
    let server_handle = start_test_server(PORT).await;

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let first_client = TcpStream::connect(format!("{}:{}", ADDRESS, PORT))
        .await
        .unwrap();

    let (first_client, response) = send_command(first_client, "AUTH admin Password4;").await;
    assert_eq!("OK;", response);

    let second_client = TcpStream::connect(format!("{}:{}", ADDRESS, PORT))
        .await
        .unwrap();

    let (second_client, response) = send_command(second_client, "AUTH admin Password4;").await;
    assert_eq!(response, "OK;");

    let (first_client, response) = send_command(first_client, "SET key value;").await;
    assert_eq!(response, "OK;");

    let (second_client, response) = send_command(second_client, "GET key;").await;
    assert_eq!(response, "value;");

    let (second_client, response) = send_command(second_client, "SET key value2;").await;
    assert_eq!(response, "OK;");

    let (first_client, response) = send_command(first_client, "GET key;").await;
    assert_eq!(response, "value2;");

    let (first_client, response) = send_command(first_client, "DEL key;").await;
    assert_eq!(response, "OK;");

    let (_, response) = send_command(first_client, "GET key;").await;
    assert_eq!(response, "Key not found;");

    let (_, response) = send_command(second_client, "GET key;").await;
    assert_eq!(response, "Key not found;");

    server_handle.abort();
}

#[tokio::test]
async fn test_integration_lot_clients() {
    const PORT: u16 = 65502;
    let server_handle = start_test_server(PORT).await;

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let mut handles = Vec::new();

    for i in 0..10 {
        let client = TcpStream::connect(format!("{}:{}", ADDRESS, PORT))
            .await
            .unwrap();

        let handle = tokio::spawn(async move {
            let (client, response) = send_command(client, "AUTH admin Password4;").await;
            assert_eq!(response, "OK;");

            let (client, response) = send_command(client, &format!("SET key{} value;", i)).await;
            assert_eq!(response, "OK;", "SET command response mismatch");

            let (client, response) = send_command(client, &format!("GET key{};", i)).await;
            assert_eq!(
                response, "value;",
                "GET command (after SET) response mismatch"
            );

            let (client, response) = send_command(client, &format!("DEL key{};", i)).await;
            assert_eq!(response, "OK;", "DEL command response mismatch");

            let (_, response) = send_command(client, &format!("GET key{};", i)).await;
            assert_eq!(
                response, "Key not found;",
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

#[tokio::test]
async fn test_integration_batch_commands() {
    const PORT: u16 = 65503;

    let _ = start_test_server(PORT).await;

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let client = TcpStream::connect(format!("{}:{}", ADDRESS, PORT))
        .await
        .unwrap();

    let (client, response) = send_command(client, "AUTH admin Password4;").await;
    assert_eq!(response, "OK;");

    let (client, response) =
        send_command(client, "SET key1 value1;SET key2 value2;SET key3 value3;").await;
    assert_eq!(response, "OK;OK;OK;");

    let (_, response) = send_command(client, "GET key1;GET key2;GET key3;").await;
    assert_eq!(response, "value1;value2;value3;");
}

#[tokio::test]
async fn test_integration_incomplete_command() {
    const PORT: u16 = 65505;

    let _ = start_test_server(PORT).await;

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let client = TcpStream::connect(format!("{}:{}", ADDRESS, PORT))
        .await
        .unwrap();

    let (client, response) = send_command(client, "AUTH admin Password4;").await;
    assert_eq!(response, "OK;");

    let (client, response) = send_command(client, "SET key value").await;
    assert_eq!(response, " ");

    let (client, response) = send_command(client, ";").await;
    assert_eq!(response, "OK;");

    let (client, response) = send_command(client, "GET key;").await;
    assert_eq!(response, "value;");

    let (client, response) = send_command(client, "SET ").await;
    assert_eq!(response, " ");

    let (_, response) = send_command(client, "key1 value1; SET key2 value2;").await;
    assert_eq!(response, "OK;OK;");
}
