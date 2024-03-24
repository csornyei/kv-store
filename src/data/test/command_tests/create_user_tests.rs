use std::str::FromStr;

use crate::{commands::Command, data::test::data_tests_utils::*, session::Session};

#[tokio::test]
async fn test_command_create_user_success() {
    let mut data = create_data_manager().await;

    let cmd = Command::from_str("CREATE_USER user Password4").unwrap();

    let (result, _) = data.handle_command(cmd, create_session()).await.unwrap();

    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("AUTH user Password4").unwrap();

    let (result, session) = data.handle_command(cmd, create_session()).await.unwrap();

    assert_eq!(result, "OK".to_string());
    assert_eq!(session.username, "user".to_string());
    assert_eq!(session.is_authenticated, true);
}

#[tokio::test]
async fn test_command_create_user_no_more_permission() {
    let mut data = create_data_manager().await;
    let cmd = Command::from_str("CREATE_USER user Password4 USER_ADMIN").unwrap();
    let (result, _) = data.handle_command(cmd, create_session()).await.unwrap();

    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("AUTH user Password4").unwrap();
    let (result, session) = data.handle_command(cmd, create_session()).await.unwrap();

    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("CREATE_USER user2 Password4 GET").unwrap();
    let result = data.handle_command(cmd, session).await.unwrap_err();

    assert_eq!(result, "User does not have permission".to_string());
}

#[tokio::test]
async fn test_command_create_user_check_auth() {
    let mut data = create_data_manager().await;

    let cmd = Command::from_str("CREATE_USER user Password4").unwrap();
    let result = data.handle_command(cmd, Session::new()).await.unwrap_err();

    assert_eq!(result, "User not authenticated".to_string());

    data.handle_command(
        Command::from_str("CREATE_USER user Password4 USER_ADMIN").unwrap(),
        create_session(),
    )
    .await
    .unwrap();

    let (_, session) = data
        .handle_command(
            Command::from_str("AUTH user Password4").unwrap(),
            Session::new(),
        )
        .await
        .unwrap();

    let cmd = Command::from_str("CREATE_USER user2 Password4").unwrap();
    let (result, _) = data.handle_command(cmd, session).await.unwrap();

    assert_eq!(result, "OK".to_string());
}

#[tokio::test]
async fn test_command_create_user_permission() {
    let mut data = create_data_manager().await;

    data.handle_command(
        Command::from_str("CREATE_USER user Password4").unwrap(),
        create_session(),
    )
    .await
    .unwrap();

    let (_, session) = data
        .handle_command(
            Command::from_str("AUTH user Password4").unwrap(),
            Session::new(),
        )
        .await
        .unwrap();

    let cmd = Command::from_str("CREATE_USER user2 Password4").unwrap();

    let result_err = data.handle_command(cmd, session.clone()).await.unwrap_err();

    assert_eq!(result_err, "User does not have permission".to_string());

    data.handle_command(
        Command::from_str("GRANT user USER_ADMIN").unwrap(),
        create_session(),
    )
    .await
    .unwrap();

    let cmd = Command::from_str("CREATE_USER user2 Password4").unwrap();
    let (result, _) = data.handle_command(cmd, session).await.unwrap();

    assert_eq!(result, "OK".to_string());
}
