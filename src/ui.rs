
use owo_colors::OwoColorize;
use crate::{CliError, Command};

pub struct ColoredUI;

impl ColoredUI {
    pub fn show_help(app_name: &str, version: &str, description: &str, command: &Command) {
        println!("{}", Self::format_help(app_name, version, description, command));
    }

    pub fn show_error(error: &CliError) {
        eprintln!("{} {}", "[ERROR]".bold().red().on_black(), error);
    }

    pub fn show_success(message: &str) {
        println!("{}, {}", "[SUCCESS]".bold().green().on_black(), message);
    }

    pub fn show_warning(message: &str) {
        println!("{}, {}", "[WARNING]".bold().yellow().on_black(), message);
    }

    pub fn show_info(message: &str) {
        println!("{}, {}", "[INFO]".bold().blue().on_black(), message);
    }

    fn format_help(app_name: &str, version: &str, description: &str, command: &Command) -> String {
        let mut help = String::new();

        help.push_str(&format!("{} v{}", app_name, version).cyan().to_string());

        if !description.is_empty() {
            help.push_str(&format!("\n\n{}", description));
        }

        help.push_str("\n\n");

        // Uso
        help.push_str(&"USO".bold().yellow().to_string());

        let usage = Self::format_usage(app_name, command);
        help.push_str(&format!("\n    {}\n\n", usage));

        // Argumentos posicionais
        if command.has_positional_args() {
            help.push_str(&"ARGUMENTOS".yellow().to_string());
            help.push('\n');

            for arg in &command.positional_args {
                let req_marker = if arg.required { "" } else { " (opcional)" };
                help.push_str(&format!("    {}{}\n{}", arg.name.green(), req_marker, arg.description));
            }
            help.push('\n');
        }

        // Flags
        if command.has_flags() {
            help.push_str(&format!("{}\n", "OPÇÕES:".yellow().bold().to_string()));

            let flags = command.get_flags_sorted();
            for flag in flags {
                let short_part = if let Some(short) = flag.short {
                    format!("-{} ", short)
                }else {
                    "    ".to_string()
                };

                let type_hint = match flag.flag_type {
                    crate::flag::FlagType::Bool => "".to_string(),
                    _ => format!("<{}>", flag.flag_type.description())
                };

                let required_maker = if flag.required { "" } else { " (opcional)" };

                help.push_str(&format!("    {} --{}{}\n        {}{}",
                                       short_part, flag.name, type_hint, flag.description, required_maker));

                if let Some(ref possible) = flag.possible_values {
                    help.push_str(&format!("        Valores possíveis: {}\n", possible.join(", ")));
                }

                if let Some(ref default) = flag.default_value {
                    help.push_str(&format!("        Padrão: {:?}\n", default));
                }
            }

            help.push('\n');
        }

        help
    }

    fn format_usage(app_name: &str, command: &Command) -> String {
        let mut usage = app_name.to_string();

        if command.name != app_name {
            usage.push_str(&format!(" {}", command.name));
        }

        if command.has_subcommands() {
            usage.push_str(" <SUBCOMANDO>");
        }

        if command.has_flags() {
            usage.push_str(" [OPÇÕES]");
        }

        for arg in &command.positional_args {
            if arg.required {
                usage.push_str(&format!(" <{}>", arg.name));
            }else {
                usage.push_str(&format!(" [{}]", arg.name));

            }
        }

        usage
    }
}

#[cfg(test)]
mod tests {
    use crate::Command;
    use crate::ui::ColoredUI;

    #[test]
    fn test_format_help_basic() {
        let command = Command::new("test");
        let help = ColoredUI::format_help(
            "myapp",
            "1.0.0",
            "Uma descrição de testes",
            &command
        );

        assert!(help.contains("myapp v1.0.0"));
        assert!(help.contains("Uma descrição de testes"));
        assert!(help.contains("USO"));
    }
}