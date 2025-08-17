use crate::{Command, Flag, ParsedArgs};

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

    pub fn parse<I, S>(&self, args: I) -> Result<ParsedArgs>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let args: Vec<String> = args.into_iter()
            .map(|s| s.as_ref().to_string())
            .collect();

        self.parse_from_args()
    }

    fn parse_from_args(&self, args: Vec<String>) -> Result<ParsedArgs> {
        CLIParser::parse(&self.root_command, args)
    }
}