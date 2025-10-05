#[cfg(test)]
mod integration_tests {
    use cliparser::{CLIApp, Command, Flag, FlagType, flag::FlagValue};

    fn create_test_app() -> CLIApp {
        CLIApp::new("test-app", "1.0.0")
            .description("Aplicação de testes")
            .add_global_flag(
                Flag::new("verbose", FlagType::Bool)
                    .short('v')
                    .description("Modo verboso"),
            )
            .add_global_flag(
                Flag::new("config", FlagType::String)
                    .short('c')
                    .description("Arquivo de configuração")
                    .default_value(FlagValue::String("default.toml".to_string())),
            )
            .add_command(
                Command::new("hello")
                    .description("Comando de saudação")
                    .add_flag(
                        Flag::new("name", FlagType::String)
                            .short('n')
                            .description("Nome para saudar")
                            .required(true),
                    )
                    .add_flag(
                        Flag::new("times", FlagType::Integer)
                            .short('t')
                            .description("Número de repetições")
                            .default_value(FlagValue::Integer(1)),
                    )
                    .add_flag(
                        Flag::new("languages", FlagType::StringList)
                            .description("Idiomas para saudação"),
                    ),
            )
            .add_command(
                Command::new("math")
                    .description("Operações matemáticas")
                    .add_subcommand(
                        Command::new("add").description("Soma números").add_flag(
                            Flag::new("numbers", FlagType::IntegerList)
                                .description("Números para somar") 
                                .required(true),
                        ),
                    )
                    .add_subcommand(
                        Command::new("subtract")
                            .description("Subtrai números")
                            .add_flag(
                                Flag::new("from", FlagType::Integer)
                                    .description("Número base")
                                    .required(true),
                            )
                            .add_flag(
                                Flag::new("value", FlagType::Integer)
                                    .description("Valor a subtrair")
                                    .required(true),
                            ),
                    ),
            )
    }

    #[test]
    fn test_app_creation() {
        let app = create_test_app();
        assert_eq!(app.name, "test-app");
        assert_eq!(app.version, "1.0.0");
        assert_eq!(app.description, "Aplicação de testes");
        assert!(app.validate().is_ok());
    }

    #[test]
    fn test_global_flags() {
        let app = create_test_app();

        let result = app.parse(vec!["--verbose"]);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.get_flag("verbose").unwrap().as_bool().unwrap(), true);

        let result = app.parse(vec!["hello", "--name", "Teste"]);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(
            parsed.get_flag("config").unwrap().as_string().unwrap(),
            "default.toml"
        )
    }

    #[test]
    fn test_simple_command() {
        let app = create_test_app();

        let result = app.parse(vec!["hello", "--name", "João"]);
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(parsed.command, "test-app");
        assert_eq!(parsed.subcommand, Some("hello".to_string()));
        assert_eq!(
            parsed.get_flag("name").unwrap().as_string().unwrap(),
            "João"
        );
        assert_eq!(parsed.get_flag("times").unwrap().as_integer().unwrap(), 1);
    }

    #[test]
    fn test_command_with_short_flags() {
        let app = create_test_app();

        let result = app.parse(vec!["hello", "-n", "Maria", "-t", "3"]);
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(
            parsed.get_flag("name").unwrap().as_string().unwrap(),
            "Maria"
        );
        assert_eq!(parsed.get_flag("times").unwrap().as_integer().unwrap(), 3);
    }

    #[test]
    fn test_list_flags() {
        let app = create_test_app();

        let result = app.parse(vec![
            "hello",
            "--name",
            "João",
            "--languages",
            "português",
            "--languages",
            "inglês",
        ]);
        assert!(result.is_ok());

        let parsed = result.unwrap();
        let languages = parsed
            .get_flag("languages")
            .unwrap()
            .as_string_list()
            .unwrap();
        assert_eq!(languages.len(), 2);
        assert!(languages.contains(&"português".to_string()));
        assert!(languages.contains(&"inglês".to_string()));
    }

    #[test]
    fn test_subcommands() {
        let app = create_test_app();

        let result: Result<cliparser::ParsedArgs, cliparser::CliError> = app.parse(vec!["math", "add", "--numbers", "1,2,3"]);
        assert!(result.is_ok(), "parse falhou com erro {:?}", result.unwrap_err());

        let parsed = result.unwrap();
        assert_eq!(parsed.subcommand, Some("math".to_string()));

        let numbers = parsed
            .get_flag("numbers")
            .unwrap()
            .as_integer_list()
            .unwrap();
        assert_eq!(numbers, &vec![1, 2, 3]);
    }

    #[test]
    fn test_required_flag_missing() {
        let app = create_test_app().show_help_on_empty(false);

        let result = app.parse(vec!["hello", "--times", "3"]);
        assert!(result.is_err(), "parse não retornou um erro, retornou: {:?}", result.unwrap());

        match result.unwrap_err() {
            cliparser::CliError::RequiredFlagNotProvided { flag } => {
                assert_eq!(flag, "name");
            }
            _ => panic!("Erro esperado: RequiredFlagMissing"),
        }
    }

    #[test]
    fn test_unknown_flag() {
        let app = create_test_app();

        let result = app.parse(vec!["hello", "--unknown", "value"]);
        assert!(result.is_err());

        match result.unwrap_err() {
            cliparser::CliError::UnknownFlag { flag } => {
                assert_eq!(flag, "unknown");
            }
            _ => panic!("Erro esperado: UnknownFlag"),
        }
    }

    #[test]
    fn test_unknown_command() {
        let app = create_test_app();

        let result = app.parse(vec!["unknown-command"]);
        assert!(result.is_err(), "O parse não retornou um erro, retornou {:?}", result);

        match result.unwrap_err() {
            cliparser::CliError::CommandNotFound { command } => {
                assert_eq!(command, "unknown-command");
            }
            err => panic!("Erro esperado: CommandNotFound, erro recebido: {:?}", err),
        }
    }

    #[test]
    fn test_help_request() {
        let app = create_test_app();

        // Help global
        let result = app.parse(vec!["--help"]);
        assert!(result.is_ok());
        assert!(result.unwrap().help_requested);

        // Help em comando
        let result = app.parse(vec!["hello", "--help"]);
        assert!(result.is_ok());
        assert!(result.unwrap().help_requested);

        // Help com flag curta
        let result = app.parse(vec!["-h"]);
        assert!(result.is_ok());
        assert!(result.unwrap().help_requested);
    }

    #[test]
    fn test_flag_value_validation() {
        let app = CLIApp::new("test", "1.0.0").add_command(
            Command::new("test-cmd").add_flag(
                Flag::new("choice", FlagType::String)
                    .possible_values(vec!["a".to_string(), "b".to_string(), "c".to_string()])
                    .required(true),
            ),
        );

        // Valor válido
        let result = app.parse(vec!["test-cmd", "--choice", "a"]);
        assert!(result.is_ok());

        // Valor inválido
        let result = app.parse(vec!["test-cmd", "--choice", "invalid"]);
        assert!(result.is_err());

        match result.unwrap_err() {
            cliparser::CliError::InvalidFlagValue {
                flag,
                value,
                expected,
            } => {
                assert_eq!(flag, "choice");
                assert_eq!(value, "invalid");
                assert!(expected.contains("a, b, c"));
            }
            _ => panic!("Erro esperado: InvalidFlagValue"),
        }
    }

    #[test]
    fn test_integer_parsing() {
        let app = CLIApp::new("test", "1.0.0").add_command(
            Command::new("test-cmd")
                .add_flag(Flag::new("number", FlagType::Integer).required(true)),
        );

        // Número válido
        let result = app.parse(vec!["test-cmd", "--number", "42"]);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.get_flag("number").unwrap().as_integer().unwrap(), 42);

        // Número inválido
        let result = app.parse(vec!["test-cmd", "--number", "not-a-number"]);
        assert!(result.is_err());

        match result.unwrap_err() {
            cliparser::CliError::InvalidFlagValue {
                flag,
                value,
                expected,
            } => {
                assert_eq!(flag, "number");
                assert_eq!(value, "not-a-number");
                assert_eq!(expected, "integer");
            }
            _ => panic!("Erro esperado: InvalidFlagValue"),
        }
    }

    #[test]
    fn test_float_parsing() {
        let app = CLIApp::new("test", "1.0.0").add_command(
            Command::new("test-cmd").add_flag(Flag::new("ratio", FlagType::Float).required(true)),
        );

        let result = app.parse(vec!["test-cmd", "--ratio", "3.14"]);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.get_flag("ratio").unwrap().as_float().unwrap(), 3.14);
    }

    #[test]
    fn test_boolean_flags() {
        let app = CLIApp::new("test", "1.0.0").add_command(
            Command::new("test-cmd")
                .add_flag(Flag::new("enable", FlagType::Bool))
                .add_flag(Flag::new("disable", FlagType::Bool)),
        );

        let result = app.parse(vec!["test-cmd", "--enable"]);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.get_flag("enable").unwrap().as_bool().unwrap(), true);
        assert!(parsed.get_flag("disable").is_none());
    }

    #[test]
    fn test_missing_flag_value() {
        let app = CLIApp::new("test", "1.0.0")
            .add_command(Command::new("test-cmd").add_flag(Flag::new("name", FlagType::String)));

        let result = app.parse(vec!["test-cmd", "--name"]);
        assert!(result.is_err());

        match result.unwrap_err() {
            cliparser::CliError::FlagValueMissing { flag } => {
                assert_eq!(flag, "name");
            }
            _ => panic!("Erro esperado: FlagValueMissing"),
        }
    }

    #[test]
    fn test_app_validation() {
        // App válido
        let valid_app = create_test_app();
        assert!(valid_app.validate().is_ok());

        // App com flag obrigatória e valor padrão (deve falhar)
        let invalid_app = CLIApp::new("invalid", "1.0.0").add_command(
            Command::new("cmd").add_flag(
                Flag::new("bad-flag", FlagType::String)
                    .required(true)
                    .default_value(FlagValue::String("default".to_string())),
            ),
        );

        assert!(invalid_app.validate().is_err());
    }

    #[test]
    fn test_app_info() {
        let app = create_test_app();
        let info = app.get_info();

        assert_eq!(info.name, "test-app");
        assert_eq!(info.version, "1.0.0");
        assert!(info.commands.contains(&"hello".to_string()));
        assert!(info.commands.contains(&"math add".to_string()));
        assert!(info.commands.contains(&"math subtract".to_string()));
        assert_eq!(info.global_flags, 2);
    }

    #[test]
    fn test_complex_scenario() {
        let app = create_test_app();

        // Comando complexo com flags globais e locais
        let result = app.parse(vec![
            "--verbose",
            "--config",
            "custom.toml",
            "hello",
            "--name",
            "João",
            "--times",
            "3",
            "--languages",
            "português",
            "--languages",
            "inglês",
            "--languages",
            "espanhol",
        ]);

        assert!(result.is_ok());
        let parsed = result.unwrap();

        // Verifica flags globais
        assert_eq!(parsed.get_flag("verbose").unwrap().as_bool().unwrap(), true);
        assert_eq!(
            parsed.get_flag("config").unwrap().as_string().unwrap(),
            "custom.toml"
        );

        // Verifica comando e flags locais
        assert_eq!(parsed.subcommand, Some("hello".to_string()));
        assert_eq!(
            parsed.get_flag("name").unwrap().as_string().unwrap(),
            "João"
        );
        assert_eq!(parsed.get_flag("times").unwrap().as_integer().unwrap(), 3);

        let languages = parsed
            .get_flag("languages")
            .unwrap()
            .as_string_list()
            .unwrap();
        assert_eq!(languages.len(), 3);
        assert!(languages.contains(&"português".to_string()));
        assert!(languages.contains(&"inglês".to_string()));
        assert!(languages.contains(&"espanhol".to_string()));
    }

    #[test]
    fn test_empty_args_help() {
        let app: CLIApp = create_test_app();

        // Por padrão deve mostrar help quando não há argumentos
        let result = app.parse(Vec::<String>::new());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().help_requested, true);
    }

    #[test]
    fn test_run_method_error_handling() {
        let app = create_test_app();

        // Teste de erro - comando inexistente
        let result = app.run(vec!["nonexistent"]);
        assert!(result.is_err());

        // Teste de sucesso - help
        let result = app.run(vec!["--help"]);
        assert!(result.is_ok());
        assert!(result.unwrap().help_requested);

        // Teste de sucesso - comando válido
        let result = app.run(vec!["hello", "--name", "Test"]);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert!(!parsed.help_requested);
        assert_eq!(parsed.subcommand, Some("hello".to_string()));
    }

    #[test]
    fn test_flag_combinations() {
        let app = CLIApp::new("test", "1.0.0").add_command(
            Command::new("cmd")
                .add_flag(Flag::new("flag1", FlagType::String).short('a'))
                .add_flag(Flag::new("flag2", FlagType::Integer).short('b'))
                .add_flag(Flag::new("flag3", FlagType::Bool).short('c')),
        );

        // Mistura de flags longas e curtas
        let result = app.parse(vec!["cmd", "--flag1", "value1", "-b", "42", "-c"]);
        assert!(result.is_ok());

        let parsed = result.unwrap();
        assert_eq!(
            parsed.get_flag("flag1").unwrap().as_string().unwrap(),
            "value1"
        );
        assert_eq!(parsed.get_flag("flag2").unwrap().as_integer().unwrap(), 42);
        assert_eq!(parsed.get_flag("flag3").unwrap().as_bool().unwrap(), true);
    }
}
