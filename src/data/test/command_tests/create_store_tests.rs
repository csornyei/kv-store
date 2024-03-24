use std::str::FromStr;

use crate::{commands::Command, data::test::data_tests_utils::*};

#[tokio::test]
async fn test_command_create_store_simple() {
    let mut data = create_data_manager().await;
    let admin_session = create_session();

    let cmd = Command::from_str("CREATE_STORE store").unwrap();

    let (result, _) = data.handle_command(cmd, admin_session).await.unwrap();

    assert_eq!(result, "OK".to_string());
}

#[tokio::test]
async fn test_command_create_store_nested() {
    let mut data = create_data_manager().await;
    let admin_session = create_session();

    let cmd = Command::from_str("CREATE_STORE store").unwrap();

    data.handle_command(cmd, admin_session.clone())
        .await
        .unwrap();

    let cmd = Command::from_str("CREATE_STORE store:substore").unwrap();

    let (result, _) = data.handle_command(cmd, admin_session).await.unwrap();

    assert_eq!(result, "OK".to_string());
}
