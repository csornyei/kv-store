use std::str::FromStr;

use crate::{commands::Command, data::test::data_tests_utils::*};

#[test]
fn test_command_create_store_simple() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("CREATE_STORE store").unwrap();

    let (result, _) = data.handle_command(cmd, admin_session).unwrap();

    assert_eq!(result, "OK".to_string());
}

#[test]
fn test_command_create_store_nested() {
    let mut data = create_data_manager();
    let admin_session = create_session();

    let cmd = Command::from_str("CREATE_STORE store").unwrap();

    data.handle_command(cmd, admin_session.clone()).unwrap();

    let cmd = Command::from_str("CREATE_STORE store:substore").unwrap();

    let (result, _) = data.handle_command(cmd, admin_session).unwrap();

    assert_eq!(result, "OK".to_string());
}
