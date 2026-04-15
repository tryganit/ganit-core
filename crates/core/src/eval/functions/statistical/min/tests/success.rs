use super::super::*;
use crate::types::Value;

#[test]
fn min_of_numbers() {
    assert_eq!(
        min_fn(&[Value::Number(5.0), Value::Number(1.0), Value::Number(3.0)]),
        Value::Number(1.0)
    );
}

#[test]
fn min_single_number() {
    assert_eq!(min_fn(&[Value::Number(7.0)]), Value::Number(7.0));
}

#[test]
fn min_ignores_text() {
    // MIN(1, "text", 3) → 1
    assert_eq!(
        min_fn(&[Value::Number(1.0), Value::Text("text".to_string()), Value::Number(3.0)]),
        Value::Number(1.0)
    );
}
