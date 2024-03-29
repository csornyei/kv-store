use std::str::FromStr;

use crate::{commands::Command, data::test::data_tests_utils::*, session::Session};

#[tokio::test]
async fn test_command_grant_number() {
    let mut data = create_data_manager().await;
    let admin_session = create_session();

    let cmd = Command::from_str("CREATE_USER user Password4 0").unwrap();
    let (result, _) = data
        .handle_command(cmd, admin_session.clone())
        .await
        .unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("GRANT user 1").unwrap();
    let (result, _) = data
        .handle_command(cmd, admin_session.clone())
        .await
        .unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("GET_USER user").unwrap();
    let (result, _) = data
        .handle_command(cmd, admin_session.clone())
        .await
        .unwrap();

    assert_eq!(result, "User: user Permissions: 1".to_string());
}

#[tokio::test]
async fn test_command_grant_name() {
    let mut data = create_data_manager().await;
    let admin_session = create_session();

    let cmd = Command::from_str("CREATE_USER user Password4 0").unwrap();
    let (result, _) = data
        .handle_command(cmd, admin_session.clone())
        .await
        .unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("GRANT user USER_ADMIN").unwrap();
    let (result, _) = data
        .handle_command(cmd, admin_session.clone())
        .await
        .unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("GET_USER user").unwrap();
    let (result, _) = data
        .handle_command(cmd, admin_session.clone())
        .await
        .unwrap();

    assert_eq!(result, "User: user Permissions: 8".to_string());
}

#[tokio::test]
async fn test_command_grant_name_multiple() {
    let mut data = create_data_manager().await;
    let admin_session = create_session();

    let cmd = Command::from_str("CREATE_USER user Password4 0").unwrap();
    let (result, _) = data
        .handle_command(cmd, admin_session.clone())
        .await
        .unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("GRANT user USER_ADMIN GET SET").unwrap();
    let (result, _) = data
        .handle_command(cmd, admin_session.clone())
        .await
        .unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("GET_USER user").unwrap();
    let (result, _) = data
        .handle_command(cmd, admin_session.clone())
        .await
        .unwrap();

    assert_eq!(result, "User: user Permissions: 11".to_string());
}

#[tokio::test]
async fn test_command_grant_not_more_than_users() {
    let mut data = create_data_manager().await;
    let admin_session = create_session();

    let cmd = Command::from_str("CREATE_USER user Password4 USER_ADMIN SET").unwrap();
    let (result, _) = data
        .handle_command(cmd, admin_session.clone())
        .await
        .unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("CREATE_USER user2 Password4").unwrap();
    let (result, _) = data
        .handle_command(cmd, admin_session.clone())
        .await
        .unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("AUTH user Password4").unwrap();
    let (result, user_session) = data.handle_command(cmd, Session::new()).await.unwrap();

    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("GRANT user2 GET").unwrap();
    let result = data
        .handle_command(cmd, user_session.clone())
        .await
        .unwrap_err();

    assert_eq!(result, "User does not have permission".to_string());
}
