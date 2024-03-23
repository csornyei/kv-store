extern crate kvstore;

use kvstore::persistence::Persistence;
use std::io::{Read, Write};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use lazy_static::lazy_static;
use tempfile::NamedTempFile;

use kvstore::data::{DataManager, Store};
use kvstore::start_server;

const ADDRESS: &str = "127.0.0.1";

lazy_static! {
    static ref PORT_TRACKER: Arc<Mutex<u16>> = Arc::new(Mutex::new(65500));
}

async fn get_next_port() -> u16 {
    let mut port = PORT_TRACKER.lock().await;
    let current_port = *port;
    *port += 1;
    current_port
}

async fn start_test_server(port: u16, file_path: Option<String>) -> tokio::task::JoinHandle<()> {
    // let persistence = match file_path {
    //     Some(path) => Persistence::new_json_file(path),
    //     None => Persistence::new_in_memory(),
    // };
    tokio::spawn(async move {
        let data = Arc::new(Mutex::new(Store::new(".".to_string())));
        // let data = Arc::new(Mutex::new(
        //     DataManager::new("admin".to_string(), "Password4".to_string(), persistence)
        //         .expect("Failed to create data manager!"),
        // ));
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
    let port = get_next_port().await;

    let server_handle = start_test_server(port, None).await;

    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    let client = TcpStream::connect(format!("{}:{}", ADDRESS, port))
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
    let port = get_next_port().await;
    let server_handle = start_test_server(port, None).await;

    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    let first_client = TcpStream::connect(format!("{}:{}", ADDRESS, port))
        .await
        .unwrap();

    let (first_client, response) = send_command(first_client, "AUTH admin Password4;").await;
    assert_eq!("OK;", response);

    let second_client = TcpStream::connect(format!("{}:{}", ADDRESS, port))
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
    let port = get_next_port().await;
    let server_handle = start_test_server(port, None).await;

    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    let mut handles = Vec::new();

    for i in 0..10 {
        let client = TcpStream::connect(format!("{}:{}", ADDRESS, port))
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
    let port = get_next_port().await;

    let _ = start_test_server(port, None).await;

    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    let client = TcpStream::connect(format!("{}:{}", ADDRESS, port))
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
    let port = get_next_port().await;

    let _ = start_test_server(port, None).await;

    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    let client = TcpStream::connect(format!("{}:{}", ADDRESS, port))
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

#[tokio::test]
async fn test_integration_persistence_save_to_json() {
    let port = get_next_port().await;

    let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");

    let file_path = temp_file.path().to_str().expect("No path").to_string();

    let _ = start_test_server(port, Some(file_path.clone())).await;

    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    let client = TcpStream::connect(format!("{}:{}", ADDRESS, port))
        .await
        .unwrap();

    let (client, response) = send_command(client, "AUTH admin Password4;").await;

    assert_eq!(response, "OK;");

    let (client, response) = send_command(client, "SET test_key test_value;").await;

    assert_eq!(response, "OK;");

    let (client, response) = send_command(client, "CREATE_STORE users;").await;

    assert_eq!(response, "OK;");

    let (client, response) = send_command(client, "CREATE_STORE users:john_doe;").await;

    assert_eq!(response, "OK;");

    let (_, response) = send_command(client, "SET users:john_doe:age 42 INT;").await;

    assert_eq!(response, "OK;");

    let mut buf = String::new();

    temp_file
        .read_to_string(&mut buf)
        .expect("Failed to read from file");

    assert_eq!(buf, "{\"name\":\".\",\"data\":{\"test_key\":{\"value\":\"test_value\",\"data_type\":\"STRING\"}},\"stores\":{\"users\":{\"name\":\"users\",\"data\":{},\"stores\":{\"john_doe\":{\"name\":\"john_doe\",\"data\":{\"age\":{\"value\":\"42\",\"data_type\":\"INT\"}},\"stores\":{}}}}}}");
}

#[tokio::test]
async fn test_integration_persistence_load_from_json() {
    let port = get_next_port().await;

    let mut temp_file = NamedTempFile::new().expect("Failed to create temporary file");

    let data = "{\"name\":\".\",\"data\":{\"test_key\":{\"value\":\"test_value\",\"data_type\":\"STRING\"}},\"stores\":{\"users\":{\"name\":\"users\",\"data\":{},\"stores\":{\"john_doe\":{\"name\":\"john_doe\",\"data\":{\"age\":{\"value\":\"42\",\"data_type\":\"INT\"}},\"stores\":{}}}}}}";

    temp_file
        .write_all(data.as_bytes())
        .expect("Failed to write to file");

    let file_path = temp_file.path().to_str().expect("No path").to_string();

    let _ = start_test_server(port, Some(file_path.clone())).await;

    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    let client = TcpStream::connect(format!("{}:{}", ADDRESS, port))
        .await
        .unwrap();

    let (client, response) = send_command(client, "AUTH admin Password4;").await;

    assert_eq!(response, "OK;");

    let (client, response) = send_command(client, "GET test_key;").await;

    assert_eq!(response, "test_value;");

    let (client, response) = send_command(client, "GET users:john_doe:age;").await;

    assert_eq!(response, "42;");

    let (_, response) = send_command(client, "GET users:john_doe:age;").await;

    assert_eq!(response, "42;");
}
