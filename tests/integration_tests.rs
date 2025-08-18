use cliparser::{CLIApp};

#[cfg(test)]
mod integration_tests {
    use cliparser::{Flag, FlagType};
    use cliparser::flag::FlagValue;
    use super::*;

    fn create_test_app() -> CLIApp {
        CLIApp::new("test-app", "1.0.0")
            .description("Aplicação de testes")
            .add_global_flag(
                Flag::new("verbose", FlagType::Bool)
                    .short('v')
                    .description("Modo verboso")
            )
            .add_global_flag(
                Flag::new("config", FlagType::String)
                    .short('c')
                    .description("Arquivo de configuração")
                    .default_value(FlagValue::String("default.toml".to_string()))
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
}