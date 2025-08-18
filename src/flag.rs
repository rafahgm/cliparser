use crate::CliError;

/// Tipos de valores que uma flag pode aceitar
#[derive(Debug, Clone)]
pub enum FlagType {
    /// Flag booleana (--verbose, --help)
    Bool,
    /// String (--name "Rafael")
    String,
    /// Numero inteiro (--age 25)
    Integer,
    /// Numero real (--price 10.5)
    Float,
    /// Lista de strings (--tags "tag1" "tag2")
    StringList,
    /// Lista de inteiros (--ids 1 2 3)
    IntegerList
}

impl FlagType {
    /// Retorna uma descrição legível do tipo
    pub fn description(&self) -> &'static str {
        match self {
            FlagType::Bool => "boolean",
            FlagType::String => "string",
            FlagType::Float => "float",
            FlagType::Integer => "integer",
            FlagType::StringList => "string list",
            FlagType::IntegerList => "integer list"
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FlagValue {
    Bool(bool),
    String(String),
    Float(f64),
    Integer(i64),
    StringList(Vec<String>),
    IntegerList(Vec<i64>)
}

impl FlagValue {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            FlagValue::String(s) => Some(s),
            _ => None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            FlagValue::Bool(b) => Some(*b),
            _ => None
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            FlagValue::Integer(i) => Some(*i),
            _ => None
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            FlagValue::Float(f) => Some(*f),
            _ => None
        }
    }

    pub fn as_string_list(&self) -> Option<&Vec<String>> {
        match self {
            FlagValue::StringList(list) => Some(list),
            _ => None
        }
    }

    pub fn as_integer_list(&self) -> Option<&Vec<i64>> {
        match self {
            FlagValue::IntegerList(list) => Some(list),
            _ => None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Flag {
    pub name: String,
    pub short: Option<char>,
    pub flag_type: FlagType,
    pub description: String,
    pub required: bool,
    pub default_value: Option<FlagValue>,
    pub possible_values: Option<Vec<String>>
}

impl Flag {
    pub fn new(name: impl Into<String>, flag_type: FlagType) -> Self {
        Self {
            name: name.into(),
            short: None,
            flag_type,
            description: String::new(),
            required: false,
            default_value: None,
            possible_values: None
        }
    }

    pub fn short(mut self, short: char) -> Self {
        self.short = Some(short);
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub fn default_value(mut self, default_value: FlagValue) -> Self {
        self.default_value = Some(default_value);
        self
    }

    pub fn possible_values(mut self, possible_values: Vec<String>) -> Self {
        self.possible_values = Some(possible_values);
        self
    }

    pub fn parse_value(&self, value: &str) -> Result<FlagValue, CliError> {
        match self.flag_type {
            FlagType::Bool => Ok(FlagValue::Bool(value.parse().unwrap_or_else(|_| value.is_empty()))),
            FlagType::String => {
                self.validate_possible_values(value)?;
                Ok(FlagValue::String(value.to_string()))
            },
            FlagType::Integer => {
                let parsed: i64 = value.parse().map_err(|_| CliError::InvalidFlagValue {
                    flag: self.name.clone(),
                    value: value.to_string(),
                    expected: "integer".to_string()
                })?;
                Ok(FlagValue::Integer(parsed))
            },
            FlagType::Float => {
                let parsed: f64 = value.parse().map_err(|_| CliError::InvalidFlagValue {
                    flag: self.name.clone(),
                    value: value.to_string(),
                    expected: "float".to_string()
                })?;
                Ok(FlagValue::Float(parsed))
            },
            FlagType::StringList => Ok(FlagValue::StringList(vec![value.to_string()])),
            FlagType::IntegerList => {
                let parsed: i64 = value.parse().map_err(|_| CliError::InvalidFlagValue {
                    flag: self.name.clone(),
                    value: value.to_string(),
                    expected: "integer".to_string()
                })?;
                Ok(FlagValue::IntegerList(vec![parsed]))
            }
        }
    }

    pub fn parse_values(&self, values: &[String]) -> Result<FlagValue, CliError> {
        match self.flag_type {
            FlagType::StringList => {
                for value in values {
                    self.validate_possible_values(value)?;
                }
                Ok(FlagValue::StringList(values.to_vec()))
            },
            FlagType::IntegerList => {
                let mut parse_values = Vec::new();
                for value in values {
                    let parsed: i64 = value.parse().map_err(|_| CliError::InvalidFlagValue {
                        flag: self.name.clone(),
                        value: value.clone(),
                        expected: "integer".to_string()
                    })?;
                    parse_values.push(parsed);
                }
                Ok(FlagValue::IntegerList(parse_values))
            },
            _ => {
                if values.len() > 1 {
                    return Err(CliError::InvalidFlagValue {
                        flag: self.name.clone(),
                        value: values.join(", "),
                        expected: format!("single {}", self.flag_type.description())
                    })
                }
                self.parse_value(&values[0])
            }
        }
    }

    fn validate_possible_values(&self, value: &str) -> Result<(), CliError> {
        if let Some(ref possible) = self.possible_values {
            if !possible.contains(&value.to_string()) {
                return Err(CliError::InvalidFlagValue {
                    flag: self.name.clone(),
                    value: value.to_string(),
                    expected: format!("one of {:?}", possible.join(", "))
                });
            }
        }
        Ok(())
    }
}