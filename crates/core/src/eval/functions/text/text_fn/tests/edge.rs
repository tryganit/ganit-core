use super::super::*;
use crate::types::Value;

#[test]
fn negative_number_integer_format() {
    assert_eq!(
        text_fn(&[Value::Number(-5.7), Value::Text("0".to_string())]),
        Value::Text("-5".to_string())
    );
}

#[test]
fn zero_integer_format() {
    assert_eq!(
        text_fn(&[Value::Number(0.0), Value::Text("0".to_string())]),
        Value::Text("0".to_string())
    );
}

#[test]
fn bool_coerced_to_number() {
    assert_eq!(
        text_fn(&[Value::Bool(true), Value::Text("0".to_string())]),
        Value::Text("1".to_string())
    );
}
