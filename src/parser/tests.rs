use crate::flag::FlagValue;
use crate::parser::CLIParser;
use crate::{AppError, Command, Flag, FlagType};

#[test]
fn test_parse_simple_command() {
    let command = Command::new("test").add_flag(Flag::new("name", FlagType::String).required(true));

    let result = CLIParser::parse(&command, vec!["--name".to_string(), "rafael".to_string()]);
    assert!(result.is_ok());

    let parsed = result.unwrap();
    assert_eq!(parsed.command, "test");
    assert_eq!(parsed.flags.len(), 1);
    assert_eq!(
        parsed.flags.get("name").unwrap(),
        &FlagValue::String("rafael".to_string())
    );
    assert!(parsed.positional_args.is_empty());
    assert!(!parsed.help_requested);
}

#[test]
fn test_parse_bool_flag() {
    let command = Command::new("test").add_flag(Flag::new("verbose", FlagType::Bool).short('v'));

    let result = CLIParser::parse(&command, vec!["-v".to_string()]);
    assert!(result.is_ok());

    let parsed = result.unwrap();
    assert_eq!(parsed.command, "test");
    assert_eq!(parsed.flags.len(), 1);
    assert_eq!(parsed.flags.get("verbose").unwrap(), &FlagValue::Bool(true));
    assert!(parsed.positional_args.is_empty());
    assert!(!parsed.help_requested);
}

#[test]
fn test_help_requested() {
    let command = Command::new("test");
    let result = CLIParser::parse(&command, vec!["--help".to_string()]);
    assert!(result.is_ok());
    assert!(result.unwrap().help_requested);
}

#[test]
fn test_missing_required_flag() {
    let command = Command::new("test")
        .add_flag(Flag::new("required", FlagType::String).required(true))
        .show_help_on_empty(false);

    let result = CLIParser::parse(&command, vec![]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    println!("{:?}", err);
    assert_eq!(
        err,
        AppError::RequiredFlagNotProvided {
            flag: "required".to_string()
        }
    );
}

#[test]
fn test_unknown_flag() {
    let command = Command::new("test");
    let result = CLIParser::parse(&command, vec!["--unknown".to_string()]);
    assert!(result.is_err());
    let err = result.unwrap_err();
    println!("{:?}", err);
    assert_eq!(
        err,
        AppError::UnknownFlag {
            flag: "unknown".to_string()
        }
    );
}
