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
    IntegerList,
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
            FlagType::IntegerList => "integer list",
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
    IntegerList(Vec<i64>),
}

impl FlagValue {
    pub fn as_string(&self) -> Option<&str> {
        match self {
            FlagValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            FlagValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            FlagValue::Integer(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            FlagValue::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn as_string_list(&self) -> Option<&Vec<String>> {
        match self {
            FlagValue::StringList(list) => Some(list),
            _ => None,
        }
    }

    pub fn as_integer_list(&self) -> Option<&Vec<i64>> {
        match self {
            FlagValue::IntegerList(list) => Some(list),
            _ => None,
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
    pub possible_values: Option<Vec<String>>,
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
            possible_values: None,
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
        println!("flag: {} flag_value: {:?}", self.name, value);
        match self.flag_type {
            FlagType::Bool => Ok(FlagValue::Bool(
                value.parse().unwrap_or_else(|_| !value.is_empty()),
            )),
            FlagType::String => {
                self.validate_possible_values(value)?;
                Ok(FlagValue::String(value.to_string()))
            }
            FlagType::Integer => {
                let parsed: i64 = value.parse().map_err(|_| CliError::InvalidFlagValue {
                    flag: self.name.clone(),
                    value: value.to_string(),
                    expected: "integer".to_string(),
                })?;
                Ok(FlagValue::Integer(parsed))
            }
            FlagType::Float => {
                let parsed: f64 = value.parse().map_err(|_| CliError::InvalidFlagValue {
                    flag: self.name.clone(),
                    value: value.to_string(),
                    expected: "float".to_string(),
                })?;
                Ok(FlagValue::Float(parsed))
            }
            FlagType::StringList => Ok(FlagValue::StringList(vec![value.to_string()])),
            FlagType::IntegerList => {
                // Divide string na virgula
                let parsed: Result<Vec<i64>, _> =
                    value.split(',').map(|s| s.trim().parse::<i64>()).collect();

                match parsed {
                    Ok(list) => Ok(FlagValue::IntegerList(list)),
                    Err(_) => Err(CliError::InvalidFlagValue {
                        flag: self.name.clone(),
                        value: value.to_string(),
                        expected: "comma-separated integers".to_string(),
                    }),
                }
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
            }
            FlagType::IntegerList => {
                let mut parse_values = Vec::new();
                for value in values {
                    let parsed: i64 = value.parse().map_err(|_| CliError::InvalidFlagValue {
                        flag: self.name.clone(),
                        value: value.clone(),
                        expected: "integer".to_string(),
                    })?;
                    parse_values.push(parsed);
                }
                Ok(FlagValue::IntegerList(parse_values))
            }
            _ => {
                if values.len() > 1 {
                    return Err(CliError::InvalidFlagValue {
                        flag: self.name.clone(),
                        value: values.join(", "),
                        expected: format!("single {}", self.flag_type.description()),
                    });
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
                    expected: format!("one of {:?}", possible.join(", ")),
                });
            }
        }
        Ok(())
    }
}

mod tests {
    use crate::{flag::FlagValue, CliError, Flag, FlagType};

    #[test]
    fn test_new_flag() {
        let flag = Flag::new("test", FlagType::String);
        assert_eq!(flag.name, "test");
        assert!(matches!(flag.flag_type, FlagType::String));
        assert_eq!(flag.short, None);
        assert_eq!(flag.description, "");
        assert!(!flag.required);
        assert!(flag.default_value.is_none());
        assert!(flag.possible_values.is_none());
    }

    #[test]
    fn test_flag_builder_methods() {
        let flag = Flag::new("test", FlagType::String)
            .short('t')
            .description("Descrição de teste")
            .required(true)
            .default_value(FlagValue::String("default".to_string()))
            .possible_values(vec!["a".to_string(), "b".to_string()]);

        assert_eq!(flag.name, "test");
        assert_eq!(flag.short, Some('t'));
        assert_eq!(flag.description, "Descrição de teste");
        assert!(flag.required);
        assert_eq!(
            flag.default_value,
            Some(FlagValue::String("default".to_string()))
        );
        assert_eq!(
            flag.possible_values,
            Some(vec!["a".to_string(), "b".to_string()])
        );
    }

    #[test]
    fn test_parse_bool_value_true() {
        let flag = Flag::new("verbose", FlagType::Bool);
        let result = flag.parse_value("true");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), FlagValue::Bool(true));
    }

    #[test]
    fn test_parse_bool_value_false() {
        let flag = Flag::new("verbose", FlagType::Bool);
        let result = flag.parse_value("false");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), FlagValue::Bool(false));
    }

    #[test]
    fn test_parse_bool_value_empty() {
        let flag = Flag::new("verbose", FlagType::Bool);
        let result = flag.parse_value("");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), FlagValue::Bool(false));
    }

    #[test]
    fn test_parse_string_value_with_possible_values() {
        let flag = Flag::new("mode", FlagType::String)
            .possible_values(vec!["dev".to_string(), "prod".to_string()]);

        let result = flag.parse_value("dev");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), FlagValue::String("dev".to_string()));
    }

    #[test]
    fn test_parse_string_value_invalid_possible_value() {
        let flag = Flag::new("mode", FlagType::String)
            .possible_values(vec!["dev".to_string(), "prod".to_string()]);

        let result = flag.parse_value("test");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CliError::InvalidFlagValue { .. }
        ));
    }

    #[test]
    fn test_parse_string_value() {
        let flag = Flag::new("name", FlagType::String);
        let result = flag.parse_value("rafael");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), FlagValue::String("rafael".to_string()));
    }

    #[test]
    fn test_parse_integer_value() {
        let flag = Flag::new("age", FlagType::Integer);
        let result = flag.parse_value("25");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), FlagValue::Integer(25));
    }

    #[test]
    fn test_parse_integer_value_negative() {
        let flag = Flag::new("age", FlagType::Integer);
        let result = flag.parse_value("-10");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), FlagValue::Integer(-10));
    }

    #[test]
    fn test_parse_integer_value_invalid() {
        let flag = Flag::new("age", FlagType::Integer);
        let result = flag.parse_value("invalid");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CliError::InvalidFlagValue { .. }
        ));
    }

    #[test]
    fn test_parse_float_value() {
        let flag = Flag::new("price", FlagType::Float);
        let result = flag.parse_value("10.5");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), FlagValue::Float(10.5));
    }

    #[test]
    fn test_parse_float_value_negative() {
        let flag = Flag::new("price", FlagType::Float);
        let result = flag.parse_value("-3.14");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), FlagValue::Float(-3.14));
    }

    #[test]
    fn test_parse_float_value_invalid() {
        let flag = Flag::new("price", FlagType::Float);
        let result = flag.parse_value("invalid");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CliError::InvalidFlagValue { .. }
        ));
    }

    #[test]
    fn test_parse_string_list_value() {
        let flag = Flag::new("tags", FlagType::StringList);
        let result = flag.parse_value("tag1");
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            FlagValue::StringList(vec!["tag1".to_string()])
        );
    }

    #[test]
    fn test_parse_integer_list_value() {
        let flag = Flag::new("ids", FlagType::IntegerList);
        let result = flag.parse_value("42");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), FlagValue::IntegerList(vec![42]));
    }

    #[test]
    fn test_parse_integer_list_value_invalid() {
        let flag = Flag::new("ids", FlagType::IntegerList);
        let result = flag.parse_value("invalid");
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CliError::InvalidFlagValue { .. }
        ));
    }

    #[test]
    fn test_parse_values_string_list() {
        let flag = Flag::new("tags", FlagType::StringList);
        let values = vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()];
        let result = flag.parse_values(&values);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), FlagValue::StringList(values));
    }

    #[test]
    fn test_parse_values_integer_list() {
        let flag = Flag::new("ids", FlagType::IntegerList);
        let values = vec!["1".to_string(), "2".to_string(), "3".to_string()];
        let result = flag.parse_values(&values);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), FlagValue::IntegerList(vec![1, 2, 3]));
    }

    #[test]
    fn test_parse_values_integer_list_with_invalid() {
        let flag = Flag::new("ids", FlagType::IntegerList);
        let values = vec!["1".to_string(), "invalid".to_string(), "3".to_string()];
        let result = flag.parse_values(&values);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CliError::InvalidFlagValue { .. }
        ));
    }

    #[test]
    fn test_parse_values_single_value_type_with_multiple_values() {
        let flag = Flag::new("name", FlagType::String);
        let values = vec!["value1".to_string(), "value2".to_string()];
        let result = flag.parse_values(&values);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            CliError::InvalidFlagValue { .. }
        ));
    }
}
