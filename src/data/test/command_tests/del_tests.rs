use std::str::FromStr;

use crate::{commands::Command, data::test::data_tests_utils::*, session::Session};

#[tokio::test]
async fn test_command_del_simple_key() {
    let mut data = create_data_manager().await;
    let admin_session = create_session();

    let cmd = Command::from_str("SET key value").unwrap();

    data.handle_command(cmd, admin_session.clone())
        .await
        .unwrap();

    let cmd = Command::from_str("DEL key").unwrap();

    let (result, session) = data.handle_command(cmd, admin_session).await.unwrap();

    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("GET key").unwrap();

    let result_err = data.handle_command(cmd, session).await.unwrap_err();

    assert_eq!(result_err, "Key not found".to_string());
}

#[tokio::test]
async fn test_command_del_key_in_store() {
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

    let cmd = Command::from_str("DEL store:key").unwrap();

    let (result, session) = data.handle_command(cmd, admin_session).await.unwrap();

    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("GET store:key").unwrap();

    let result_err = data.handle_command(cmd, session).await.unwrap_err();

    assert_eq!(result_err, "Key not found".to_string());
}

#[tokio::test]
async fn test_command_del_key_in_embedded_store() {
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

    let cmd = Command::from_str("DEL store:embedded:key").unwrap();

    let (result, session) = data.handle_command(cmd, admin_session).await.unwrap();

    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("GET store:embedded:key").unwrap();

    let result_err = data.handle_command(cmd, session).await.unwrap_err();

    assert_eq!(result_err, "Key not found".to_string());
}

#[tokio::test]
async fn test_command_del_delete_store() {
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

    let cmd = Command::from_str("DEL store").unwrap();

    let (result, session) = data.handle_command(cmd, admin_session).await.unwrap();

    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("GET store:key").unwrap();

    let result_err = data.handle_command(cmd, session).await.unwrap_err();

    assert_eq!(result_err, "Key not found".to_string());
}

#[tokio::test]
async fn test_command_del_delete_embedded_store() {
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

    let cmd = Command::from_str("SET store:key store_value").unwrap();

    data.handle_command(cmd, admin_session.clone())
        .await
        .unwrap();

    let cmd = Command::from_str("DEL store:embedded").unwrap();

    let (result, session) = data.handle_command(cmd, admin_session).await.unwrap();

    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("GET store:key").unwrap();

    let (result, session) = data.handle_command(cmd, session).await.unwrap();

    assert_eq!(result, "store_value".to_string());

    let cmd = Command::from_str("GET store:embedded:key").unwrap();

    let result_err = data.handle_command(cmd, session).await.unwrap_err();

    assert_eq!(result_err, "Key not found".to_string());
}

#[tokio::test]
async fn test_command_del_permission() {
    let mut data = create_data_manager().await;

    data.handle_command(
        Command::from_str("CREATE_USER user Password4 GET").unwrap(),
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

    let cmd = Command::from_str("DEL key").unwrap();

    let result_err = data.handle_command(cmd, session.clone()).await.unwrap_err();
    assert_eq!(result_err, "User does not have permission".to_string());

    data.handle_command(
        Command::from_str("GRANT user DEL").unwrap(),
        create_session(),
    )
    .await
    .unwrap();

    let cmd = Command::from_str("DEL key").unwrap();
    let (result, session) = data.handle_command(cmd, session).await.unwrap();

    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("GET key").unwrap();
    let result = data.handle_command(cmd, session).await.unwrap_err();

    assert_eq!(result, "Key not found".to_string());
}

#[tokio::test]
async fn test_command_del_check_auth() {
    let mut data = create_data_manager().await;
    let session = Session::new();

    data.handle_command(
        Command::from_str("SET key value").unwrap(),
        create_session(),
    )
    .await
    .unwrap();

    let cmd = Command::from_str("DEL key").unwrap();
    let result = data.handle_command(cmd, session.clone()).await.unwrap_err();

    assert_eq!(result, "User not authenticated".to_string());

    let cmd = Command::from_str("CREATE_USER user Password4 GET DEL").unwrap();
    data.handle_command(cmd, create_session()).await.unwrap();

    let cmd = Command::from_str("AUTH user Password4").unwrap();
    let (_, session) = data.handle_command(cmd, session).await.unwrap();

    let cmd = Command::from_str("DEL key").unwrap();

    let (result, session) = data.handle_command(cmd, session).await.unwrap();

    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("GET key").unwrap();
    let result = data.handle_command(cmd, session).await.unwrap_err();

    assert_eq!(result, "Key not found".to_string());
}
