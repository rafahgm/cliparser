use crate::flag::FlagValue;
use crate::{AppError, Command, FlagType};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ParsedArgs {
    pub command: String,
    pub subcommand: Option<String>,
    pub flags: HashMap<String, FlagValue>,
    pub positional_args: Vec<String>,
    pub help_requested: bool,
}

impl ParsedArgs {
    pub fn new(command: String) -> Self {
        Self {
            command,
            subcommand: None,
            flags: HashMap::new(),
            positional_args: Vec::new(),
            help_requested: false,
        }
    }

    pub fn get_flag(&self, flag: &str) -> Option<&FlagValue> {
        self.flags.get(flag)
    }

    pub fn has_flag(&self, flag: &str) -> bool {
        self.flags.contains_key(flag)
    }

    pub fn get_arg(&self, arg: usize) -> Option<&String> {
        self.positional_args.get(arg)
    }

    pub fn get_args(&self) -> &Vec<String> {
        &self.positional_args
    }
}

pub struct CLIParser;

impl CLIParser {
    pub fn parse(command: &Command, args: Vec<String>) -> Result<ParsedArgs, AppError> {
        let mut parsed = ParsedArgs::new(command.name.clone());
        let mut i = 0;

        if args.is_empty() && command.show_help_on_empty {
            parsed.help_requested = true;
            return Ok(parsed);
        }

        while i < args.len() {
            let arg = &args[i];

            if arg == "--help" || arg == "-h" {
                parsed.help_requested = true;
                return Ok(parsed);
            }

            if arg.starts_with("--") {
                let flag_name: &str = &arg[2..];
                i += Self::parse_long_flag(command, &args, &mut i, flag_name, &mut parsed)?;
            } else if arg.starts_with("-") && arg.len() == 2 {
                let flag_char = arg.chars().nth(1).unwrap();
                i += Self::parse_short_flag(command, &args, &mut i, flag_char, &mut parsed)?;
            } else if let Some(subcommand) = command.get_subcommand(arg) {
                parsed.subcommand = Some(arg.clone());
                let remaining_args = args[i + 1..].to_vec();
                let sub_parsed = Self::parse(subcommand, remaining_args)?;

                parsed.flags.extend(sub_parsed.flags);
                parsed.positional_args.extend(sub_parsed.positional_args);
                parsed.help_requested = sub_parsed.help_requested;

                break;
            } else if command.has_positional_args() {
                parsed.positional_args.push(arg.clone());
                i += 1;
            }else {
                return Err(AppError::CommandNotFound { command: arg.clone() })
            }
        }

        if command.has_positional_args() {
            Self::validate_positional_args(command, &parsed)?;
        }

        Self::apply_defaults_and_validate(command, &mut parsed)?;

        Ok(parsed)
    }

    fn parse_long_flag(
        command: &Command,
        args: &[String],
        i: &mut usize,
        flag_name: &str,
        parsed: &mut ParsedArgs,
    ) -> Result<usize, AppError> {
        let flag = command
            .get_flag(flag_name)
            .ok_or_else(|| AppError::UnknownFlag {
                flag: flag_name.to_string(),
            })?;

        match flag.flag_type {
            FlagType::Bool => {
                parsed
                    .flags
                    .insert(flag_name.to_string(), FlagValue::Bool(true));
                Ok(1)
            }
            _ => {
                if *i + 1 >= args.len() {
                    return Err(AppError::FlagValueMissing {
                        flag: flag_name.to_string(),
                    });
                }

                *i += 1;
                let value = flag.parse_value(&args[*i])?;

                if matches!(flag.flag_type, FlagType::StringList | FlagType::IntegerList) {
                    if let Some(existing) = parsed.flags.get(&flag.name) {
                        let combined = Self::combine_list_values(existing, &value)?;
                        parsed.flags.insert(flag.name.clone(), combined);
                    } else {
                        parsed.flags.insert(flag.name.clone(), value);
                    }
                } else {
                    parsed.flags.insert(flag.name.clone(), value);
                }

                Ok(1)
            }
        }
    }

    fn parse_short_flag(
        command: &Command,
        args: &[String],
        i: &mut usize,
        flag_char: char,
        parsed: &mut ParsedArgs,
    ) -> Result<usize, AppError> {
        let flag = command
            .flags
            .values()
            .find(|f| f.short == Some(flag_char))
            .ok_or_else(|| AppError::UnknownFlag {
                flag: flag_char.to_string(),
            })?;

        Self::parse_long_flag(command, args, i, &flag.name, parsed)
    }

    fn combine_list_values(existing: &FlagValue, new: &FlagValue) -> Result<FlagValue, AppError> {
        match (existing, new) {
            (FlagValue::StringList(existing_list), FlagValue::StringList(new_list)) => {
                let mut combined = existing_list.clone();
                combined.extend(new_list.clone());
                Ok(FlagValue::StringList(combined))
            }
            (FlagValue::IntegerList(existing_list), FlagValue::IntegerList(new_list)) => {
                let mut combined = existing_list.clone();
                combined.extend(new_list.clone());
                Ok(FlagValue::IntegerList(combined))
            }
            _ => Err(AppError::ParseError {
                message: "Erro interno: tentativa de combinar valores incompativeis".to_string(),
            }),
        }
    }

    fn validate_positional_args(command: &Command, parsed: &ParsedArgs) -> Result<(), AppError> {
        let required_count = command.required_positional_count();
        let provided_count = parsed.positional_args.len();

        if provided_count < required_count {
            return Err(AppError::NotEnoughArguments {
                expected: required_count,
                received: provided_count,
            });
        }

        if provided_count > command.positional_args.len() {
            return Err(AppError::TooManyArguments);
        }

        Ok(())
    }

    fn apply_defaults_and_validate(
        command: &Command,
        parsed: &mut ParsedArgs,
    ) -> Result<(), AppError> {

        for flag in command.flags.values() {
            if !parsed.flags.contains_key(&flag.name) {
                if let Some(default_value) = &flag.default_value {
                    parsed
                        .flags
                        .insert(flag.name.clone(), default_value.clone());
                } else if flag.required {
                    return Err(AppError::RequiredFlagNotProvided {
                        flag: flag.name.clone(),
                    });
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests;

