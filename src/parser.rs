use std::collections::HashMap;
use crate::{CliError, Command};
use crate::flag::FlagValue;

#[derive(Debug, Clone)]
pub struct ParsedArgs {
    pub command: String,
    pub subcommand: Option<String>,
    pub flags: HashMap<String, FlagValue>,
    pub positional_args: Vec<String>,
    pub help_requested: bool
}

impl ParsedArgs {
    pub fn new(command: String) -> Self {
        Self {
            command,
            subcommand: None,
            flags: HashMap::new(),
            positional_args: Vec::new(),
            help_requested: false
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
    pub fn parse(command: &Command, args: Vec<String>) -> Result<ParsedArgs> {
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
                let flag_name = &arg[2..];
                i += Self::parse_long_flag(command, &args, &mut i, flag_name, &mut parsed)?;
            }else if arg.starts_with("-") && arg.len() == 2 {
                let flag_char = arg.chars().nth(1).unwrap();
                i += Self::parse_short_flag(command, &args, &mut i, flag_char, &mut parsed)?;
            }
        }
    }

    fn parse_long_flag(
        command: &Command,
        args: &[String],
        i: &mut usize,
        flag_name: &str,
        parsed: &mut ParsedArgs
    ) -> Result<usize> {
        let flag = command.get_flag(flag_name).ok_or_else(|| CliError::UnknowFlag {
            flag: flag_name.to_string()
        });

        match flag.flag_type {
            FlagValue::Bool => {
                parsed.flags.insert(flag_name.to_string(), FlagValue::Bool(true));
                Ok(1)
            }
            _ => {
                if *i + 1 >= args.len() {
                    return Err(CliError::FlagValueMissing {
                        flag: flag_name.to_string()
                    });
                }

                *i += 1;
                let value = flag.unwrap().parse_value(&args[*i])?;

                if matches!(flag.flag_type, FlagValue::StringList  | FlagValue::IntegerList) {
                    if let Some(existing) = parsed.flags.get(&flag.name) {
                        let combined  = Self::combine_list_values(existing, &value)?;
                        parsed.flags.insert(flag.name.clone(), combined);
                    }else {
                        parsed.flags.insert(flag.name.clone(), value);
                    }
                }else {
                    parsed.flags.insert(flag.name.clone(), value);
                }

                Ok(1)
            }
        }
    }

    fn parse_short_flag() -> Result<usize> {
        
    }
}