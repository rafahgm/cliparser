use crate::{CliError, Command, Flag, ParsedArgs};
use crate::parser::CLIParser;

#[derive(Debug, Clone)]
pub struct CLIApp {
    pub name: String,
    pub version: String,
    pub description: String,
    pub root_command: Command
}

impl CLIApp {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        let name = name.into();
        let root_command = Command::new(name.clone());

        Self {
            name,
            version: version.into(),
            description: String::new(),
            root_command
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn add_command(mut self, command: Command) -> Self {
        self.root_command = self.root_command.add_subcommand(command);
        self
    }

    pub fn add_global_flag(mut self, flag: Flag) -> Self {
        self.root_command = self.root_command.add_flag(flag);
        self
    }

    pub fn parse<I, S>(&self, args: I) -> Result<ParsedArgs, CliError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let args: Vec<String> = args.into_iter()
            .map(|s| s.as_ref().to_string())
            .collect();

        self.parse_from_args(args)
    }

    fn parse_from_args(&self, args: Vec<String>) -> Result<ParsedArgs, CliError> {
        CLIParser::parse(&self.root_command, args)
    }

    pub fn validate(&self) -> Result<(), CliError> {
        self.validate_command(&self.root_command)
    }

    fn validate_command(&self, command: &Command) -> Result<(), CliError> {
        let mut flag_names = std::collections::HashSet::new();
        let mut short_names = std::collections::HashSet::new();

        for flag in command.flags.values() {
            if !flag_names.insert(flag.name.clone()) {
                return Err(CliError::ConfigurationError {
                    message: format!("Flag duplicada encontrada: {}", flag.name)
                });
            }

            if let Some(short) = flag.short {
                if !short_names.insert(short) {
                    return Err(CliError::ConfigurationError {
                        message: format!("Flag curta duplicada encontrada: {}", short)
                    });
                }
            }

            if flag.required && flag.default_value.is_some() {
                return Err(CliError::ConfigurationError {
                    message: format!("Flag '{}' não pode ser obrigatória e ter valor padrão", flag.name),
                });
            }
        }

        let mut subcommand_names = std::collections::HashSet::new();
        for subcommand in command.subcommands.values() {
            if !subcommand_names.insert(subcommand.name.clone()) {
                return Err(CliError::ConfigurationError {
                    message: format!("Subcomando duplicado encontrado: {}", subcommand.name)
                });
            }

            self.validate_command(subcommand)?;
        }

        Ok(())
    }
}