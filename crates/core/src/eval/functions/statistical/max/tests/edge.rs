use super::super::*;
use crate::types::Value;

#[test]
fn max_no_args_returns_zero() {
    assert_eq!(max_fn(&[]), Value::Number(0.0));
}

#[test]
fn max_all_non_numeric_returns_zero() {
    assert_eq!(
        max_fn(&[Value::Text("a".to_string()), Value::Bool(true), Value::Empty]),
        Value::Number(0.0)
    );
}

#[test]
fn max_negative_numbers() {
    assert_eq!(
        max_fn(&[Value::Number(-3.0), Value::Number(-1.0), Value::Number(-5.0)]),
        Value::Number(-1.0)
    );
}
