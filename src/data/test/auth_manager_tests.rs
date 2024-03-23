use crate::data::auth_manager::*;

#[test]
fn test_permission_from_u8() {
    assert_eq!(Permissions::from_u8(0), vec![]);
    assert_eq!(Permissions::from_u8(1), vec![Permissions::SET]);
    assert_eq!(Permissions::from_u8(2), vec![Permissions::GET]);
    assert_eq!(
        Permissions::from_u8(3),
        vec![Permissions::SET, Permissions::GET]
    );
    assert_eq!(Permissions::from_u8(4), vec![Permissions::DEL]);
    assert_eq!(
        Permissions::from_u8(5),
        vec![Permissions::SET, Permissions::DEL]
    );
    assert_eq!(
        Permissions::from_u8(6),
        vec![Permissions::GET, Permissions::DEL]
    );
    assert_eq!(
        Permissions::from_u8(7),
        vec![Permissions::SET, Permissions::GET, Permissions::DEL]
    );
    assert_eq!(Permissions::from_u8(8), vec![Permissions::USER_ADMIN]);
    assert_eq!(
        Permissions::from_u8(9),
        vec![Permissions::SET, Permissions::USER_ADMIN]
    );
    assert_eq!(
        Permissions::from_u8(10),
        vec![Permissions::GET, Permissions::USER_ADMIN]
    );
    assert_eq!(
        Permissions::from_u8(11),
        vec![Permissions::SET, Permissions::GET, Permissions::USER_ADMIN]
    );
    assert_eq!(
        Permissions::from_u8(12),
        vec![Permissions::DEL, Permissions::USER_ADMIN]
    );
    assert_eq!(
        Permissions::from_u8(13),
        vec![Permissions::SET, Permissions::DEL, Permissions::USER_ADMIN]
    );
    assert_eq!(
        Permissions::from_u8(14),
        vec![Permissions::GET, Permissions::DEL, Permissions::USER_ADMIN]
    );
    assert_eq!(
        Permissions::from_u8(15),
        vec![
            Permissions::SET,
            Permissions::GET,
            Permissions::DEL,
            Permissions::USER_ADMIN
        ]
    );

    assert_eq!(
        Permissions::from_u8(255),
        vec![
            Permissions::SET,
            Permissions::GET,
            Permissions::DEL,
            Permissions::USER_ADMIN
        ]
    );
}

#[test]
fn test_user_to_string() {
    let user = User::new("user".to_string(), "password".to_string(), 0);

    assert_eq!(user.to_string(), "User: user Permissions: 0");
}
