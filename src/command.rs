use std::collections::HashMap;
use crate::Flag;

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub flags: HashMap<String, Flag>,
    pub subcommands: HashMap<String, Command>,
    pub positional_args: Vec<PositionalArg>,
    pub show_help_on_empty: bool
}

#[derive(Debug, Clone)]
pub struct PositionalArg {
    pub name: String,
    pub description: String,
    pub required: bool
}

impl PositionalArg {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: String::new(),
            required: true
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }
}

impl Command {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: String::new(),
            flags: HashMap::new(),
            subcommands: HashMap::new(),
            positional_args: Vec::new(),
            show_help_on_empty: false
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn add_flag(mut self, flag: Flag) -> Self {
        self.flags.insert(flag.name.clone(), flag);
        self
    }

    pub fn add_subcommand(mut self, subcommand: Command) -> Self {
        self.subcommands.insert(subcommand.name.clone(), subcommand);
        self
    }

    pub fn add_positional_arg(mut self, positional_arg: PositionalArg) -> Self {
        self.positional_args.push(positional_arg);
        self
    }

    pub fn show_help_on_empty(mut self, show_help_on_empty: bool) -> Self {
        self.show_help_on_empty = show_help_on_empty;
        self
    }

    pub fn get_flag(&self, name: &str) -> Option<&Flag> {
        if let Some(flag) = self.flags.get(name) {
            return Some(flag);
        }

        if name.len() == 1 {
            let short_char = name.chars().next().unwrap();
            for flag in self.flags.values() {
                if flag.short == Some(short_char) {
                    return Some(flag);
                }
            }
        }

        None
    }

    pub fn get_subcommand(&self, name: &str) -> Option<&Command> {
        self.subcommands.get(name)
    }

    pub fn get_flags_sorted(&self) -> Vec<&Flag> {
        let mut flags: Vec<&Flag> = self.flags.values().collect();
        flags.sort_by(|a, b| a.name.cmp(&b.name));
        flags
    }

    pub fn get_subcommands_sorted(&self) -> Vec<&Command> {
        let mut subcommands: Vec<&Command> = self.subcommands.values().collect();
        subcommands.sort_by(|a, b| a.name.cmp(&b.name));
        subcommands
    }

    pub fn has_subcommands(&self) -> bool {
        !self.subcommands.is_empty()
    }

    pub fn has_flags(&self) -> bool {
        !self.flags.is_empty()
    }

    pub fn has_positional_args(&self) -> bool {
        !self.positional_args.is_empty()
    }

    pub fn required_positional_count(&self) -> usize {
        self.positional_args.iter().filter(|arg| arg.required).count()
    }
}