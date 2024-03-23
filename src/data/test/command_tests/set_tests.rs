use std::str::FromStr;

use crate::{
    commands::Command,
    data::{test::data_tests_utils::*, DataTypes},
    session::Session,
};

#[tokio::test]
async fn test_command_set_validate_args() {
    let cmd = Command::from_str("SET key true BOOL").unwrap();

    let key = cmd.args[0].clone();

    assert_eq!(key, "key".to_string());

    let value = cmd.args[1].clone();

    assert_eq!(value, "true".to_string());

    let data_type = DataTypes::from_str(&cmd.args[2]).unwrap();

    assert_eq!(data_type, DataTypes::BOOL);
}

#[tokio::test]
async fn test_command_set_simple_key() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("SET key value").unwrap();

    let (result, _) = data.handle_command(cmd, admin_session).await.unwrap();

    assert_eq!(result, "OK".to_string());
}

#[tokio::test]
async fn test_command_set_int() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("SET key 10 INT").unwrap();

    let (result, _) = data.handle_command(cmd, admin_session).await.unwrap();

    assert_eq!(result, "OK".to_string());
}

#[tokio::test]
async fn test_command_set_float() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("SET key 10.5 FLOAT").unwrap();

    let (result, _) = data.handle_command(cmd, admin_session).await.unwrap();

    assert_eq!(result, "OK".to_string());
}

#[tokio::test]
async fn test_command_set_bool() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("SET key true BOOL").unwrap();

    let (result, _) = data.handle_command(cmd, admin_session).await.unwrap();

    assert_eq!(result, "OK".to_string());
}

#[tokio::test]
async fn test_command_set_key_in_store() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("CREATE_STORE store").unwrap();

    data.handle_command(cmd, admin_session.clone())
        .await
        .unwrap();

    let cmd = Command::from_str("SET store:key value").unwrap();

    let (result, _) = data.handle_command(cmd, admin_session).await.unwrap();

    assert_eq!(result, "OK".to_string());
}

#[tokio::test]
async fn test_command_set_key_in_embedded_store() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("CREATE_STORE store").unwrap();

    data.handle_command(cmd, admin_session.clone())
        .await
        .unwrap();

    let cmd = Command::from_str("CREATE_STORE store:substore").unwrap();

    data.handle_command(cmd, admin_session.clone())
        .await
        .unwrap();

    let cmd = Command::from_str("SET store:substore:key value").unwrap();

    let (result, _) = data.handle_command(cmd, admin_session).await.unwrap();

    assert_eq!(result, "OK".to_string());
}

#[tokio::test]
async fn test_command_set_permission() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("CREATE_USER user Password4").unwrap();

    data.handle_command(cmd, admin_session.clone())
        .await
        .unwrap();

    let cmd = Command::from_str("AUTH user Password4").unwrap();

    let (_, session) = data
        .handle_command(cmd, admin_session.clone())
        .await
        .unwrap();

    let cmd = Command::from_str("SET key value").unwrap();

    let result_err = data.handle_command(cmd, session.clone()).await.unwrap_err();

    assert_eq!(result_err, "User does not have permission".to_string());

    let cmd = Command::from_str("GRANT user SET").unwrap();
    data.handle_command(cmd, admin_session.clone())
        .await
        .unwrap();

    let cmd = Command::from_str("SET key value").unwrap();
    let (result, _) = data.handle_command(cmd, session).await.unwrap();

    assert_eq!(result, "OK".to_string());
}

#[tokio::test]
async fn test_command_set_check_auth() {
    let mut data = create_data_manager();
    let session = Session::new();

    let cmd = Command::from_str("SET key value").unwrap();
    let result = data.handle_command(cmd, session.clone()).await.unwrap_err();

    assert_eq!(result, "User not authenticated".to_string());

    let cmd = Command::from_str("CREATE_USER user Password4 SET").unwrap();
    data.handle_command(cmd, create_session()).await.unwrap();

    let cmd = Command::from_str("AUTH user Password4").unwrap();
    let (_, session) = data.handle_command(cmd, session).await.unwrap();

    let cmd = Command::from_str("SET key value").unwrap();

    let (result, _) = data.handle_command(cmd, session).await.unwrap();

    assert_eq!(result, "OK".to_string());
}
