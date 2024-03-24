use std::str::FromStr;

use crate::{commands::Command, data::test::data_tests_utils::*};

#[tokio::test]
async fn test_command_get_user() {
    let mut data = create_data_manager().await;

    let cmd = Command::from_str("GET_USER user").unwrap();
    let result_err = data
        .handle_command(cmd, create_session())
        .await
        .unwrap_err();

    assert_eq!(result_err, "User not found".to_string());

    let cmd = Command::from_str("CREATE_USER user Password4").unwrap();
    let (result, _) = data.handle_command(cmd, create_session()).await.unwrap();

    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("GET_USER user").unwrap();
    let (result, _) = data.handle_command(cmd, create_session()).await.unwrap();

    assert_eq!(result, "User: user Permissions: 0".to_string())
}
