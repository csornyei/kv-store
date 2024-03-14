use std::io::{Error, ErrorKind};

#[derive(Debug, PartialEq)]
pub enum CommandNames {
    SET,
    GET,
    DEL,
}

fn string_to_command_name(s: &str) -> Result<CommandNames, Error> {
    match s {
        "SET" => Ok(CommandNames::SET),
        "GET" => Ok(CommandNames::GET),
        "DEL" => Ok(CommandNames::DEL),
        _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid command")),
    }
}

pub struct Command {
    pub name: CommandNames,
    args: Vec<String>,
}

impl Command {
    fn new(name: CommandNames, args: Vec<String>) -> Command {
        Command { name, args }
    }

    fn validate_args(command: &CommandNames, args: Vec<String>) -> Result<(), Error> {
        match command {
            CommandNames::SET => {
                if args.len() != 2 {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Invalid number of arguments",
                    ));
                }
            }
            CommandNames::GET => {
                if args.len() != 1 {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Invalid number of arguments",
                    ));
                }
            }
            CommandNames::DEL => {
                if args.len() != 1 {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        "Invalid number of arguments",
                    ));
                }
            }
        }
        Ok(())
    }

    pub fn from_string(line: String) -> Result<Command, Error> {
        let (name, args) = parse_line(line)?;
        Command::validate_args(&name, args.clone())?;
        Ok(Command::new(name, args))
    }
}

fn parse_line(line: String) -> Result<(CommandNames, Vec<String>), Error> {
    let parts: Vec<&str> = line.trim().splitn(2, ' ').collect();
    if parts.len() < 1 {
        return Err(Error::new(ErrorKind::InvalidInput, "No command"));
    }
    if parts.len() < 2 {
        let cmd = string_to_command_name(parts[0])?;
        return Ok((cmd, Vec::new()));
    }

    let command = parts[0].to_string();
    let args: Vec<String> = parts[1].trim().split(' ').map(|s| s.to_string()).collect();

    let cmd = string_to_command_name(&command)?;
    Ok((cmd, args))
}

#[cfg(test)]
mod parser_tests {
    use super::*;

    #[test]
    fn test_string_to_commands() {
        assert_eq!(string_to_command_name("SET").unwrap(), CommandNames::SET);
        assert_eq!(string_to_command_name("GET").unwrap(), CommandNames::GET);
        assert_eq!(string_to_command_name("DEL").unwrap(), CommandNames::DEL);
        assert!(string_to_command_name("INVALID").is_err());
    }

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
    fn test_command_validate_args() {
        assert!(Command::validate_args(&CommandNames::SET, vec!["key".to_string()]).is_err());
        assert!(Command::validate_args(
            &CommandNames::SET,
            vec!["key".to_string(), "value".to_string()]
        )
        .is_ok());
        assert!(Command::validate_args(
            &CommandNames::GET,
            vec!["key".to_string(), "value".to_string()]
        )
        .is_err());
        assert!(Command::validate_args(&CommandNames::GET, vec!["key".to_string()]).is_ok());
        assert!(Command::validate_args(
            &CommandNames::DEL,
            vec!["key".to_string(), "value".to_string()]
        )
        .is_err());
        assert!(Command::validate_args(&CommandNames::DEL, vec!["key".to_string()]).is_ok());
    }

    #[test]
    fn test_command_from_string() {
        assert!(Command::from_string("SET key value".to_string()).is_ok());
        assert!(Command::from_string("SET key".to_string()).is_err());
    }
}
