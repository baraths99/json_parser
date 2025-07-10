use crate::{parse,JsonValue};
use std::collections::HashMap;

#[test]
fn test_basic_values() {
    assert_eq!(parse("null"), Ok(JsonValue::Null));
    assert_eq!(parse("true"), Ok(JsonValue::Boolean(true)));
    assert_eq!(parse("false"), Ok(JsonValue::Boolean(false)));
    assert_eq!(parse("123"), Ok(JsonValue::Number(123.0)));
    assert_eq!(parse("-45.67"), Ok(JsonValue::Number(-45.67)));
}

#[test]
fn test_strings() {
    assert_eq!(
        parse("\"hello\""),
        Ok(JsonValue::String("hello".to_string()))
    );
    assert_eq!(
        parse("\"hello\\nworld\""),
        Ok(JsonValue::String("hello\nworld".to_string()))
    );
}

#[test]
fn test_arrays() {
    assert_eq!(parse("[]"), Ok(JsonValue::Array(vec![])));

    let expected = JsonValue::Array(vec![
        JsonValue::Number(1.0),
        JsonValue::Number(2.0),
        JsonValue::Number(3.0),
    ]);
    assert_eq!(parse("[1, 2, 3]"), Ok(expected));
}

#[test]
fn test_objects() {
    assert_eq!(parse("{}"), Ok(JsonValue::Object(HashMap::new())));

    let mut map = HashMap::new();
    map.insert("name".to_string(), JsonValue::String("John".to_string()));
    map.insert("age".to_string(), JsonValue::Number(30.0));

    assert_eq!(
        parse("{\"name\": \"John\", \"age\": 30}"),
        Ok(JsonValue::Object(map))
    );
}

#[test]
fn test_nested_structures() {
    let json = r#"{"data": {"users": [{"id": 1, "name": "Alice"}]}}"#;

    match parse(json) {
        Ok(JsonValue::Object(map)) => {
            assert!(map.contains_key("data"));
        }
        _ => panic!("Failed to parse"),
    }
}

#[test]
fn test_errors() {
    assert!(parse("{").is_err());
    assert!(parse("\"unclosed string").is_err());
    assert!(parse("{\"key\": 1,}").is_err());
}
