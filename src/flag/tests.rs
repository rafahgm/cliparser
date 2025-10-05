use super::*;

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
        AppError::InvalidFlagValue { .. }
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
        AppError::InvalidFlagValue { .. }
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
        AppError::InvalidFlagValue { .. }
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
        AppError::InvalidFlagValue { .. }
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
        AppError::InvalidFlagValue { .. }
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
        AppError::InvalidFlagValue { .. }
    ));
}
