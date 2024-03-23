// Permissions bit mask:
// 0b00000001 - SET
// 0b00000010 - GET
// 0b00000100 - DEL
// 0b00001000 - CREATE_USER & DELETE_USER
// To GRANT permission user needs 0b00001000 & appropriate permission:
// 0b00001000 | 0b00000001 = 0b00001001
// 0b00001000 | 0b00000010 = 0b00001010
// 0b00001000 | 0b00000100 = 0b00001100

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum Permissions {
    NONE = 0,
    SET = 1 << 0,
    GET = 1 << 1,
    DEL = 1 << 2,
    USER_ADMIN = 1 << 3,
}

impl Permissions {
    pub fn from_u8(value: u8) -> Vec<Permissions> {
        let mut permissions = Vec::new();

        if value & (Permissions::SET as u8) != 0 {
            permissions.push(Permissions::SET);
        }

        if value & (Permissions::GET as u8) != 0 {
            permissions.push(Permissions::GET);
        }

        if value & (Permissions::DEL as u8) != 0 {
            permissions.push(Permissions::DEL);
        }

        if value & (Permissions::USER_ADMIN as u8) != 0 {
            permissions.push(Permissions::USER_ADMIN);
        }

        permissions
    }
}
