use std::str::FromStr;

use crate::{commands::Command, data::data_tests_utils::*, session::Session};

#[test]
fn test_command_get_simple_key() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("SET key value").unwrap();

    data.handle_command(cmd, admin_session.clone()).unwrap();

    let cmd = Command::from_str("GET key").unwrap();

    let (result, _) = data.handle_command(cmd, admin_session).unwrap();

    assert_eq!(result, "value".to_string());
}

#[test]
fn test_command_get_key_in_store() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("CREATE_STORE store").unwrap();

    data.handle_command(cmd, admin_session.clone()).unwrap();

    let cmd = Command::from_str("SET store:key value").unwrap();

    data.handle_command(cmd, admin_session.clone()).unwrap();

    let cmd = Command::from_str("GET store:key").unwrap();

    let (result, _) = data.handle_command(cmd, admin_session).unwrap();

    assert_eq!(result, "value".to_string());
}

#[test]
fn test_command_get_key_in_embedded_store() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("CREATE_STORE store").unwrap();

    data.handle_command(cmd, admin_session.clone()).unwrap();

    let cmd = Command::from_str("CREATE_STORE store:embedded").unwrap();

    data.handle_command(cmd, admin_session.clone()).unwrap();

    let cmd = Command::from_str("SET store:embedded:key value").unwrap();

    data.handle_command(cmd, admin_session.clone()).unwrap();

    let cmd = Command::from_str("GET store:embedded:key").unwrap();

    let (result, _) = data.handle_command(cmd, admin_session).unwrap();

    assert_eq!(result, "value".to_string());
}

#[test]
fn test_command_get_permission() {
    let mut data = create_data_manager();

    data.handle_command(
        Command::from_str("CREATE_USER user Password4").unwrap(),
        create_session(),
    )
    .unwrap();

    data.handle_command(
        Command::from_str("SET key value").unwrap(),
        create_session(),
    )
    .unwrap();

    let cmd = Command::from_str("AUTH user Password4").unwrap();

    let (_, session) = data.handle_command(cmd, Session::new()).unwrap();

    let cmd = Command::from_str("GET key").unwrap();

    let result_err = data.handle_command(cmd, session.clone()).unwrap_err();
    assert_eq!(result_err, "User does not have permission".to_string());

    data.handle_command(
        Command::from_str("GRANT user GET").unwrap(),
        create_session(),
    )
    .unwrap();

    let cmd = Command::from_str("GET key").unwrap();
    let (result, _) = data.handle_command(cmd, session).unwrap();

    assert_eq!(result, "value".to_string());
}

#[test]
fn test_command_set_check_auth() {
    let mut data = create_data_manager();
    let session = Session::new();

    data.handle_command(
        Command::from_str("SET key value").unwrap(),
        create_session(),
    )
    .unwrap();

    let cmd = Command::from_str("GET key").unwrap();
    let result = data.handle_command(cmd, session.clone()).unwrap_err();

    assert_eq!(result, "User not authenticated".to_string());

    let cmd = Command::from_str("CREATE_USER user Password4 GET").unwrap();
    data.handle_command(cmd, create_session()).unwrap();

    let cmd = Command::from_str("AUTH user Password4").unwrap();
    let (_, session) = data.handle_command(cmd, session).unwrap();

    let cmd = Command::from_str("GET key").unwrap();

    let (result, _) = data.handle_command(cmd, session).unwrap();

    assert_eq!(result, "value".to_string());
}
