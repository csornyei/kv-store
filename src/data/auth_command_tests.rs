use super::data_tests_utils::*;

use crate::commands::Command;
use crate::session::Session;
use std::str::FromStr;

#[test]
fn test_handle_command_auth() {
    let mut data = create_data_manager();
    let cmd = Command::from_str("AUTH user Password4").unwrap();

    let result_err = data.handle_command(cmd, create_session()).unwrap_err();

    assert_eq!(result_err, "Username or password is incorrect".to_string());

    let cmd = Command::from_str("CREATE_USER user Password4").unwrap();

    let (result, _) = data.handle_command(cmd, create_session()).unwrap();

    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("AUTH user Password4").unwrap();

    let result = data.handle_command(cmd, Session::new()).unwrap();

    assert_eq!(result.0, "OK".to_string());
    assert_eq!(result.1.username, "user".to_string());
}

#[test]
fn test_handle_command_create_user() {
    let mut data = create_data_manager();
    let cmd = Command::from_str("CREATE_USER user Password4").unwrap();
    let (result, _) = data.handle_command(cmd, create_session()).unwrap();

    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("AUTH user Password4").unwrap();
    let (result, session) = data.handle_command(cmd, create_session()).unwrap();

    assert_eq!(result, "OK".to_string());
    assert_eq!(session.username, "user".to_string());
}

#[test]
fn test_handle_command_create_user_cant_more_permission() {
    let mut data = create_data_manager();
    let cmd = Command::from_str("CREATE_USER user Password4 USER_ADMIN").unwrap();
    let (result, _) = data.handle_command(cmd, create_session()).unwrap();

    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("AUTH user Password4").unwrap();
    let (result, session) = data.handle_command(cmd, create_session()).unwrap();

    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("CREATE_USER user2 Password4 GET").unwrap();
    let result = data.handle_command(cmd, session).unwrap_err();

    assert_eq!(result, "User does not have permission".to_string());
}

#[test]
fn test_handle_command_delete_user() {
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
fn test_handle_command_get_user() {
    let mut data = create_data_manager();

    let cmd = Command::from_str("GET_USER user").unwrap();
    let result_err = data.handle_command(cmd, create_session()).unwrap_err();

    assert_eq!(result_err, "User not found".to_string());

    let cmd = Command::from_str("CREATE_USER user Password4").unwrap();
    let (result, _) = data.handle_command(cmd, create_session()).unwrap();

    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("GET_USER user").unwrap();
    let (result, _) = data.handle_command(cmd, create_session()).unwrap();

    assert_eq!(result, "User: user Permissions: 0".to_string())
}

#[test]
fn test_check_auth_commands() {
    let mut data = create_data_manager();
    let session = Session::new();

    let cmd = Command::from_str("SET key value").unwrap();
    let result = data.handle_command(cmd, session.clone()).unwrap_err();
    assert_eq!(result, "User not authenticated".to_string());

    let cmd = Command::from_str("GET key").unwrap();
    let result = data.handle_command(cmd, session.clone()).unwrap_err();
    assert_eq!(result, "User not authenticated".to_string());

    let cmd = Command::from_str("DEL key").unwrap();
    let result = data.handle_command(cmd, session.clone()).unwrap_err();
    assert_eq!(result, "User not authenticated".to_string());

    let cmd = Command::from_str("CREATE_USER user Password4").unwrap();
    let result = data.handle_command(cmd, session.clone()).unwrap_err();
    assert_eq!(result, "User not authenticated".to_string());

    let cmd = Command::from_str("DELETE_USER user").unwrap();
    let result = data.handle_command(cmd, session.clone()).unwrap_err();
    assert_eq!(result, "User not authenticated".to_string());
}

#[test]
fn test_check_auth_deleted_user() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("CREATE_USER user Password4 255").unwrap();
    let (result, _) = data.handle_command(cmd, admin_session.clone()).unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("AUTH user Password4").unwrap();
    let (result, user_session) = data.handle_command(cmd, Session::new()).unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("SET key value").unwrap();
    let (result, _) = data.handle_command(cmd, user_session.clone()).unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("GET key").unwrap();
    let (result, _) = data.handle_command(cmd, user_session.clone()).unwrap();
    assert_eq!(result, "value".to_string());

    let cmd = Command::from_str("DELETE_USER user").unwrap();
    let (result, _) = data.handle_command(cmd, admin_session.clone()).unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("GET key").unwrap();
    let result = data.handle_command(cmd, user_session.clone()).unwrap_err();
    assert_eq!(result, "User not authenticated".to_string());

    let cmd = Command::from_str("GET key").unwrap();
    let (result, _) = data.handle_command(cmd, admin_session.clone()).unwrap();
    assert_eq!(result, "value".to_string());
}

#[test]
fn test_check_auth_no_permission() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("CREATE_USER user Password4 0").unwrap();
    let (result, _) = data.handle_command(cmd, admin_session.clone()).unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("AUTH user Password4").unwrap();
    let (result, user_session) = data.handle_command(cmd, Session::new()).unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("SET key value").unwrap();
    let result = data.handle_command(cmd, user_session.clone()).unwrap_err();
    assert_eq!(result, "User does not have permission".to_string());
}

#[test]
fn test_handle_command_grant_number() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("CREATE_USER user Password4 0").unwrap();
    let (result, _) = data.handle_command(cmd, admin_session.clone()).unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("GRANT user 1").unwrap();
    let (result, _) = data.handle_command(cmd, admin_session.clone()).unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("GET_USER user").unwrap();
    let (result, _) = data.handle_command(cmd, admin_session.clone()).unwrap();

    assert_eq!(result, "User: user Permissions: 1".to_string());
}

#[test]
fn test_handle_command_grant_name() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("CREATE_USER user Password4 0").unwrap();
    let (result, _) = data.handle_command(cmd, admin_session.clone()).unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("GRANT user USER_ADMIN").unwrap();
    let (result, _) = data.handle_command(cmd, admin_session.clone()).unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("GET_USER user").unwrap();
    let (result, _) = data.handle_command(cmd, admin_session.clone()).unwrap();

    assert_eq!(result, "User: user Permissions: 8".to_string());
}

#[test]
fn test_handle_command_grant_name_multiple() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("CREATE_USER user Password4 0").unwrap();
    let (result, _) = data.handle_command(cmd, admin_session.clone()).unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("GRANT user USER_ADMIN GET SET").unwrap();
    let (result, _) = data.handle_command(cmd, admin_session.clone()).unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("GET_USER user").unwrap();
    let (result, _) = data.handle_command(cmd, admin_session.clone()).unwrap();

    assert_eq!(result, "User: user Permissions: 11".to_string());
}

#[test]
fn test_handle_command_grant_not_more_than_users() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("CREATE_USER user Password4 USER_ADMIN SET").unwrap();
    let (result, _) = data.handle_command(cmd, admin_session.clone()).unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("CREATE_USER user2 Password4").unwrap();
    let (result, _) = data.handle_command(cmd, admin_session.clone()).unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("AUTH user Password4").unwrap();
    let (result, user_session) = data.handle_command(cmd, Session::new()).unwrap();

    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("GRANT user2 GET").unwrap();
    let result = data.handle_command(cmd, user_session.clone()).unwrap_err();

    assert_eq!(result, "User does not have permission".to_string());
}

#[test]
fn test_handle_command_revoke_number() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("CREATE_USER user Password4 1").unwrap();
    let (result, _) = data.handle_command(cmd, admin_session.clone()).unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("REVOKE user 1").unwrap();
    let (result, _) = data.handle_command(cmd, admin_session.clone()).unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("GET_USER user").unwrap();
    let (result, _) = data.handle_command(cmd, admin_session.clone()).unwrap();

    assert_eq!(result, "User: user Permissions: 0".to_string());
}

#[test]
fn test_handle_command_revoke_name() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("CREATE_USER user Password4 8").unwrap();
    let (result, _) = data.handle_command(cmd, admin_session.clone()).unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("REVOKE user USER_ADMIN").unwrap();
    let (result, _) = data.handle_command(cmd, admin_session.clone()).unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("GET_USER user").unwrap();
    let (result, _) = data.handle_command(cmd, admin_session.clone()).unwrap();

    assert_eq!(result, "User: user Permissions: 0".to_string());
}

#[test]
fn test_handle_command_revoke_name_multiple() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("CREATE_USER user Password4 11").unwrap();
    let (result, _) = data.handle_command(cmd, admin_session.clone()).unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("REVOKE user USER_ADMIN GET").unwrap();
    let (result, _) = data.handle_command(cmd, admin_session.clone()).unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("GET_USER user").unwrap();
    let (result, _) = data.handle_command(cmd, admin_session.clone()).unwrap();

    assert_eq!(result, "User: user Permissions: 1".to_string());
}

#[test]
fn test_handle_command_revoke_not_more_than_users() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("CREATE_USER user Password4 USER_ADMIN SET").unwrap();
    let (result, _) = data.handle_command(cmd, admin_session.clone()).unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("CREATE_USER user2 Password4").unwrap();
    let (result, _) = data.handle_command(cmd, admin_session.clone()).unwrap();
    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("AUTH user Password4").unwrap();
    let (result, user_session) = data.handle_command(cmd, Session::new()).unwrap();

    assert_eq!(result, "OK".to_string());

    let cmd = Command::from_str("REVOKE user2 GET").unwrap();
    let result = data.handle_command(cmd, user_session.clone()).unwrap_err();

    assert_eq!(result, "User does not have permission".to_string());
}
