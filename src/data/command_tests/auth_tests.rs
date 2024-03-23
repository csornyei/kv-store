use std::str::FromStr;

use crate::{commands::Command, data::data_tests_utils::*};

#[test]
fn test_command_auth_no_user() {
    let mut data = create_data_manager();
    let cmd = Command::from_str("AUTH user Password4").unwrap();

    let result_err = data.handle_command(cmd, create_session()).unwrap_err();

    assert_eq!(result_err, "Username or password is incorrect".to_string());
}

#[test]
fn test_command_auth_wrong_password() {
    let mut data = create_data_manager();

    let cmd = Command::from_str("CREATE_USER user Password4").unwrap();

    data.handle_command(cmd, create_session()).unwrap();

    let cmd = Command::from_str("AUTH user Password1").unwrap();

    let result_err = data.handle_command(cmd, create_session()).unwrap_err();

    assert_eq!(result_err, "Username or password is incorrect".to_string());
}

#[test]
fn test_command_auth_success() {
    let mut data = create_data_manager();

    let cmd = Command::from_str("CREATE_USER user Password4").unwrap();

    data.handle_command(cmd, create_session()).unwrap();

    let cmd = Command::from_str("AUTH user Password4").unwrap();

    let (result, session) = data.handle_command(cmd, create_session()).unwrap();

    assert_eq!(result, "OK".to_string());

    assert_eq!(session.username, "user".to_string());
    assert_eq!(session.is_authenticated, true);
}
