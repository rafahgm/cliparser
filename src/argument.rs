use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArgType {
    String,
    Integer,
    Float,
    Boolean,
    Flag
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Argument {
    pub name: String,
    pub short: Option<char>,
    pub long: String,
    pub description: String,
    pub arg_type: ArgType,
    pub required: bool,
    pub default: Option<String>
}

impl Argument {
    pub fn new (name: &str, long: &str) -> Self {
        Self {
            name: name.to_string(),
            short: None,
            long: long.to_string(),
            description: String::new(),
            arg_type: ArgType::String,
            required: false,
            default: None
        }
    }

    pub fn short(mut self, short: char) -> Self {
        self.short = Some(short);
        self
    }

    pub fn description(mut self, description: &str) -> Self {
        self.description = description.to_string();
        self
    }

    pub fn arg_type(mut self, arg_type: ArgType) -> Self {
        self.arg_type = arg_type;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn default(mut self, default: &str) -> Self {
        self.default = Some(default.to_string());
        self
    }
}