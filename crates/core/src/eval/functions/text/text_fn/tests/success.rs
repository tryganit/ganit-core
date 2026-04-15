use super::super::*;
use crate::types::Value;

#[test]
fn format_zero_integer() {
    assert_eq!(
        text_fn(&[Value::Number(42.0), Value::Text("0".to_string())]),
        Value::Text("42".to_string())
    );
}

#[test]
fn format_default_float() {
    assert_eq!(
        text_fn(&[Value::Number(3.14), Value::Text("0.00".to_string())]),
        Value::Text("3.14".to_string())
    );
}

#[test]
fn format_integer_truncated() {
    assert_eq!(
        text_fn(&[Value::Number(3.9), Value::Text("0".to_string())]),
        Value::Text("3".to_string())
    );
}
