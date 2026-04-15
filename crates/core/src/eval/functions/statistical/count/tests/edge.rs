use super::super::*;
use crate::types::Value;

#[test]
fn count_no_args_returns_zero() {
    assert_eq!(count_fn(&[]), Value::Number(0.0));
}

#[test]
fn counta_no_args_returns_zero() {
    assert_eq!(counta_fn(&[]), Value::Number(0.0));
}

#[test]
fn count_mixed_ignores_non_numeric() {
    // COUNT(1, "text", TRUE, 3) → 2
    assert_eq!(
        count_fn(&[
            Value::Number(1.0),
            Value::Text("text".to_string()),
            Value::Bool(true),
            Value::Number(3.0)
        ]),
        Value::Number(2.0)
    );
}

#[test]
fn counta_mixed_counts_all_non_empty() {
    // COUNTA(1, "text", TRUE, 3) → 4
    assert_eq!(
        counta_fn(&[
            Value::Number(1.0),
            Value::Text("text".to_string()),
            Value::Bool(true),
            Value::Number(3.0)
        ]),
        Value::Number(4.0)
    );
}

#[test]
fn counta_empty_values_not_counted() {
    assert_eq!(
        counta_fn(&[Value::Empty, Value::Number(1.0), Value::Empty]),
        Value::Number(1.0)
    );
}
