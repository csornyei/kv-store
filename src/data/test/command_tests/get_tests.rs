use std::str::FromStr;

use crate::{commands::Command, data::test::data_tests_utils::*, session::Session};

#[tokio::test]
async fn test_command_get_simple_key() {
    let mut data = create_data_manager().await;
    let admin_session = create_session();

    let cmd = Command::from_str("SET key value").unwrap();

    data.handle_command(cmd, admin_session.clone())
        .await
        .unwrap();

    let cmd = Command::from_str("GET key").unwrap();

    let (result, _) = data.handle_command(cmd, admin_session).await.unwrap();

    assert_eq!(result, "value".to_string());
}

#[tokio::test]
async fn test_command_get_key_in_store() {
    let mut data = create_data_manager().await;
    let admin_session = create_session();

    let cmd = Command::from_str("CREATE_STORE store").unwrap();

    data.handle_command(cmd, admin_session.clone())
        .await
        .unwrap();

    let cmd = Command::from_str("SET store:key value").unwrap();

    data.handle_command(cmd, admin_session.clone())
        .await
        .unwrap();

    let cmd = Command::from_str("GET store:key").unwrap();

    let (result, _) = data.handle_command(cmd, admin_session).await.unwrap();

    assert_eq!(result, "value".to_string());
}

#[tokio::test]
async fn test_command_get_key_in_embedded_store() {
    let mut data = create_data_manager().await;
    let admin_session = create_session();

    let cmd = Command::from_str("CREATE_STORE store").unwrap();

    data.handle_command(cmd, admin_session.clone())
        .await
        .unwrap();

    let cmd = Command::from_str("CREATE_STORE store:embedded").unwrap();

    data.handle_command(cmd, admin_session.clone())
        .await
        .unwrap();

    let cmd = Command::from_str("SET store:embedded:key value").unwrap();

    data.handle_command(cmd, admin_session.clone())
        .await
        .unwrap();

    let cmd = Command::from_str("GET store:embedded:key").unwrap();

    let (result, _) = data.handle_command(cmd, admin_session).await.unwrap();

    assert_eq!(result, "value".to_string());
}

#[tokio::test]
async fn test_command_get_permission() {
    let mut data = create_data_manager().await;

    data.handle_command(
        Command::from_str("CREATE_USER user Password4").unwrap(),
        create_session(),
    )
    .await
    .unwrap();

    data.handle_command(
        Command::from_str("SET key value").unwrap(),
        create_session(),
    )
    .await
    .unwrap();

    let cmd = Command::from_str("AUTH user Password4").unwrap();

    let (_, session) = data.handle_command(cmd, Session::new()).await.unwrap();

    let cmd = Command::from_str("GET key").unwrap();

    let result_err = data.handle_command(cmd, session.clone()).await.unwrap_err();
    assert_eq!(result_err, "User does not have permission".to_string());

    data.handle_command(
        Command::from_str("GRANT user GET").unwrap(),
        create_session(),
    )
    .await
    .unwrap();

    let cmd = Command::from_str("GET key").unwrap();
    let (result, _) = data.handle_command(cmd, session).await.unwrap();

    assert_eq!(result, "value".to_string());
}

#[tokio::test]
async fn test_command_get_check_auth() {
    let mut data = create_data_manager().await;
    let session = Session::new();

    data.handle_command(
        Command::from_str("SET key value").unwrap(),
        create_session(),
    )
    .await
    .unwrap();

    let cmd = Command::from_str("GET key").unwrap();
    let result = data.handle_command(cmd, session.clone()).await.unwrap_err();

    assert_eq!(result, "User not authenticated".to_string());

    let cmd = Command::from_str("CREATE_USER user Password4 GET").unwrap();
    data.handle_command(cmd, create_session()).await.unwrap();

    let cmd = Command::from_str("AUTH user Password4").unwrap();
    let (_, session) = data.handle_command(cmd, session).await.unwrap();

    let cmd = Command::from_str("GET key").unwrap();

    let (result, _) = data.handle_command(cmd, session).await.unwrap();

    assert_eq!(result, "value".to_string());
}

#[tokio::test]
async fn test_command_get_whole_store() {
    let mut data = create_data_manager().await;
    let admin_session = create_session();

    data.handle_command(
        Command::from_str("CREATE_STORE test_store_name").unwrap(),
        create_session(),
    )
    .await
    .unwrap();

    data.handle_command(
        Command::from_str("SET test_store_name:key1 value1").unwrap(),
        admin_session.clone(),
    )
    .await
    .unwrap();

    data.handle_command(
        Command::from_str("SET test_store_name:key2 100 INT").unwrap(),
        admin_session.clone(),
    )
    .await
    .unwrap();

    let cmd = Command::from_str("GET test_store_name:*").unwrap();

    let (result, _) = data.handle_command(cmd, admin_session).await.unwrap();

    assert!(result.contains("key1: value1 (STRING)"));
    assert!(result.contains("key2: 100 (INT)"));
    assert!(result.contains("test_store_name"));
}
