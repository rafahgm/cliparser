use crate::{App, Command, Flag, FlagType};

#[test]
fn test_create_app() {
    let app = App::new("app", "1.0.0").description("app teste");

    assert_eq!(app.name, "app");
    assert_eq!(app.version, "1.0.0");
    assert_eq!(app.description, "app teste");
}

#[test]
fn test_parse_simple_command() {
    let mut app = App::new("app", "1.0.0");
    app = app.add_command(
        Command::new("hello")
            .description("Hello World")
            .add_flag(Flag::new("name", FlagType::String).required(true)),
    );

    let result = app.parse(vec!["hello", "--name", "Rafael"]);
    assert!(result.is_ok());

    let parsed = result.unwrap();

    assert_eq!(parsed.command, "app");
    assert_eq!(parsed.subcommand, Some("hello".to_string()));
    assert_eq!(
        parsed.get_flag("name").unwrap().as_string().unwrap(),
        "Rafael"
    );
}

#[test]
fn test_validation_duplicate_flags() {
    let app = App::new("app", "1.0.0")
        .add_global_flag(Flag::new("name", FlagType::String).required(true))
        .add_global_flag(Flag::new("name", FlagType::String).required(true));

    assert_eq!(app.root_command.flags.len(), 1);
}

#[test]
fn test_help_requested() {
    let app = App::new("app", "1.0.0");
    let result = app.parse(vec!["--help"]);
    assert!(result.is_ok());
    assert!(result.unwrap().help_requested);
}
