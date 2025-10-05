use crate::parser::CLIParser;
use crate::ui::ColoredUI;
use crate::{AppError, Command, Flag, ParsedArgs};
use std::env;
#[derive(Debug, Clone)]
pub struct App {
    pub name: String,
    pub version: String,
    pub description: String,
    pub root_command: Command,
}

impl App {
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        let name = name.into();
        let root_command =
            Command::new(name.clone());

        Self {
            name,
            version: version.into(),
            description: String::new(),
            root_command,
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

     pub fn show_help_on_empty(mut self, show_help_on_empty: bool) -> Self {
        self.root_command = self.root_command.show_help_on_empty(show_help_on_empty);
        self
    }

    pub fn parse<I, S>(&self, args: I) -> Result<ParsedArgs, AppError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let args: Vec<String> = args.into_iter().map(|s| s.as_ref().to_string()).collect();
        self.parse_from_args(args)
    }

    fn parse_from_args(&self, args: Vec<String>) -> Result<ParsedArgs, AppError> {
        CLIParser::parse(&self.root_command, args)
    }

    pub fn validate(&self) -> Result<(), AppError> {
        self.validate_command(&self.root_command)
    }

    fn validate_command(&self, command: &Command) -> Result<(), AppError> {
        let mut flag_names = std::collections::HashSet::new();
        let mut short_names = std::collections::HashSet::new();

        for flag in command.flags.values() {
            if !flag_names.insert(flag.name.clone()) {
                return Err(AppError::ConfigurationError {
                    message: format!("Flag duplicada encontrada: {}", flag.name),
                });
            }

            if let Some(short) = flag.short {
                if !short_names.insert(short) {
                    return Err(AppError::ConfigurationError {
                        message: format!("Flag curta duplicada encontrada: {}", short),
                    });
                }
            }

            if flag.required && flag.default_value.is_some() {
                return Err(AppError::ConfigurationError {
                    message: format!(
                        "Flag '{}' não pode ser obrigatória e ter valor padrão",
                        flag.name
                    ),
                });
            }
        }

        let mut subcommand_names = std::collections::HashSet::new();
        for subcommand in command.subcommands.values() {
            if !subcommand_names.insert(subcommand.name.clone()) {
                return Err(AppError::ConfigurationError {
                    message: format!("Subcomando duplicado encontrado: {}", subcommand.name),
                });
            }

            self.validate_command(subcommand)?;
        }

        Ok(())
    }

    pub fn run<I, S>(&self, args: I) -> Result<ParsedArgs, AppError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        match self.parse(args) {
            Ok(parsed) => {
                if parsed.help_requested {
                    self.show_help(&parsed);
                    return Ok(parsed);
                }
                Ok(parsed)
            }
            Err(error) => {
                ColoredUI::show_error(&error);
                if let AppError::CommandNotFound { .. }
                | AppError::SubcommandNotFound { .. }
                | AppError::UnknownFlag { .. } = error
                {
                    println!();
                    ColoredUI::show_info("Use --help para obter ajuda");
                }
                Err(error)
            }
        }
    }

    fn show_help(&self, parsed: &ParsedArgs) {
        let command = if let Some(ref subcommmand_name) = parsed.subcommand {
            if let Some(subcommand) = self.root_command.subcommands.get(subcommmand_name) {
                subcommand
            } else {
                &self.root_command
            }
        } else {
            &self.root_command
        };

        ColoredUI::show_help(&self.name, &self.version, &self.description, command);
    }

    pub fn get_info(&self) -> AppInfo {
        let mut commands = Vec::new();

        self.collect_commands(&self.root_command, String::new(), &mut commands);

        AppInfo {
            name: self.name.clone(),
            version: self.version.clone(),
            description: self.description.clone(),
            commands: commands,
            global_flags: self.root_command.flags.len(),
        }
    }

    fn collect_commands(&self, command: &Command, prefix: String, commands: &mut Vec<String>) {
        for subcommand in command.subcommands.values() {
            let full_name = if prefix.is_empty() {
                subcommand.name.clone()
            } else {
                format!("{} {}", prefix, subcommand.name)
            };

            commands.push(full_name.clone());
            self.collect_commands(subcommand, full_name, commands);
        }
    }

    pub fn run_from_env(&self) -> Result<ParsedArgs, AppError> {
        let args: Vec<String> = env::args().skip(1).collect();
        self.run(args)
    }

   
}

#[derive(Debug, Clone)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub commands: Vec<String>,
    pub global_flags: usize,
}

impl AppInfo {
    pub fn display(&self) {
        ColoredUI::show_info(&format!("{} v{}", self.name, self.version));

        if !self.description.is_empty() {
            println!("{}", self.description);
        }

        println!("Comandos disponíveis: {}", self.commands.len());
        for command in &self.commands {
            println!("  - {}", command);
        }

        println!("Flags globais: {}", self.global_flags);
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new("app", "0.0.0")
    }
}

#[cfg(test)]
mod tests;
