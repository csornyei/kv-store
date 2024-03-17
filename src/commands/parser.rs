use std::io::{Error, ErrorKind};
use std::str::FromStr;

use super::CommandNames;

pub fn parse_permissions(args: &str) -> u8 {
    let mut permissions = 0;
    for permission in args.split(' ') {
        match parse_permission_num(permission) {
            Ok(perm) => return perm,
            Err(_) => {
                let parsed_permissions = parse_permissions_str(permissions, permission);
                match parsed_permissions {
                    Ok(perm) => {
                        permissions = perm;
                    }
                    Err(_) => {
                        break;
                    }
                }
            }
        }
    }
    permissions
}

fn parse_permission_num(permissions: &str) -> Result<u8, Error> {
    match permissions.parse::<u8>() {
        Ok(perm) => Ok(perm),
        Err(_) => Err(Error::new(ErrorKind::InvalidInput, "Invalid permission")),
    }
}

fn parse_permissions_str(current_permissions: u8, permission: &str) -> Result<u8, Error> {
    match permission {
        "SET" => Ok(current_permissions | 1 << 0),
        "GET" => Ok(current_permissions | 1 << 1),
        "DEL" => Ok(current_permissions | 1 << 2),
        "USER_ADMIN" => Ok(current_permissions | 1 << 3),

        _ => return Err(Error::new(ErrorKind::InvalidInput, "Invalid permission")),
    }
}

pub fn parse_line(line: String) -> Result<(CommandNames, Vec<String>), Error> {
    let parts: Vec<&str> = line.trim().splitn(2, ' ').collect();
    if parts.len() < 1 {
        return Err(Error::new(ErrorKind::InvalidInput, "No command"));
    }
    if parts.len() < 2 {
        let cmd = CommandNames::from_str(parts[0])?;
        return Ok((cmd, Vec::new()));
    }

    let command = parts[0].to_string();
    let args: Vec<String> = parts[1].trim().split(' ').map(|s| s.to_string()).collect();

    let cmd = CommandNames::from_str(&command)?;
    Ok((cmd, args))
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn test_parse_line_no_string() {
        assert!(parse_line("".to_string()).is_err());
    }

    #[test]
    fn test_parse_line_no_args() {
        assert_eq!(
            parse_line("GET".to_string()).unwrap(),
            (CommandNames::GET, Vec::new())
        );
    }

    #[test]
    fn test_parse_line_with_args() {
        assert_eq!(
            parse_line("SET key value".to_string()).unwrap(),
            (
                CommandNames::SET,
                vec!["key".to_string(), "value".to_string()]
            )
        );
    }

    #[test]
    fn test_parse_permissions() {
        let permissions = parse_permissions("");
        assert_eq!(permissions, 0);

        let permissions = parse_permissions("SET");
        assert_eq!(permissions, 1);

        let permissions = parse_permissions("SET GET");
        assert_eq!(permissions, 3);

        let permissions = parse_permissions("SET GET DEL");

        assert_eq!(permissions, 7);

        let permissions = parse_permissions("SET GET DEL USER_ADMIN");
        assert_eq!(permissions, 15);

        let permissions = parse_permissions("SET GET DEL USER_ADMIN INVALID");
        assert_eq!(permissions, 15);

        let permissions = parse_permissions("255");
        assert_eq!(permissions, 255);
    }
}
