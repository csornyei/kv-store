use std::str::FromStr;

use crate::{commands::Command, data::test::data_tests_utils::*, session::Session};

#[test]
fn test_command_delete_user() {
    let mut data = create_data_manager();

    let cmd = Command::from_str("CREATE_USER user Password4").unwrap();
    let (result, _) = data.handle_command(cmd, create_session()).unwrap();

    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("DELETE_USER user").unwrap();

    let (result, _) = data.handle_command(cmd, create_session()).unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("AUTH user Password4").unwrap();

    let result_err = data.handle_command(cmd, create_session()).unwrap_err();

    assert_eq!(result_err, "Username or password is incorrect".to_string());
}

#[test]
fn test_command_delete_user_not_allowed_after() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("CREATE_USER user Password4 255").unwrap();
    let (result, admin_session) = data.handle_command(cmd, admin_session).unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("AUTH user Password4").unwrap();
    let (result, user_session) = data.handle_command(cmd, Session::new()).unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("SET key value").unwrap();
    let (result, user_session) = data.handle_command(cmd, user_session).unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("GET key").unwrap();
    let (result, user_session) = data.handle_command(cmd, user_session).unwrap();
    assert_eq!(result, "value".to_string());

    let cmd = Command::from_str("DELETE_USER user").unwrap();
    let (result, admin_session) = data.handle_command(cmd, admin_session).unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("GET key").unwrap();
    let result = data.handle_command(cmd, user_session).unwrap_err();
    assert_eq!(result, "User not authenticated".to_string());

    let cmd = Command::from_str("GET key").unwrap();
    let (result, _) = data.handle_command(cmd, admin_session).unwrap();
    assert_eq!(result, "value".to_string());
}
