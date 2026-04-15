use super::super::*;
use crate::types::Value;

#[test]
fn max_of_numbers() {
    assert_eq!(
        max_fn(&[Value::Number(1.0), Value::Number(5.0), Value::Number(3.0)]),
        Value::Number(5.0)
    );
}

#[test]
fn max_single_number() {
    assert_eq!(max_fn(&[Value::Number(7.0)]), Value::Number(7.0));
}

#[test]
fn max_ignores_text() {
    // MAX(1, "text", 3) → 3
    assert_eq!(
        max_fn(&[Value::Number(1.0), Value::Text("text".to_string()), Value::Number(3.0)]),
        Value::Number(3.0)
    );
}
