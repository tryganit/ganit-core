use super::super::{iferror_fn, ifna_fn};
use crate::types::{ErrorKind, Value};

#[test]
fn iferror_no_error_returns_value() {
    assert_eq!(iferror_fn(&[Value::Number(1.0), Value::Number(0.0)]), Value::Number(1.0));
}

#[test]
fn iferror_on_error_returns_fallback() {
    assert_eq!(
        iferror_fn(&[Value::Error(ErrorKind::DivByZero), Value::Number(0.0)]),
        Value::Number(0.0)
    );
}

#[test]
fn ifna_no_error_returns_value() {
    assert_eq!(ifna_fn(&[Value::Number(42.0), Value::Number(0.0)]), Value::Number(42.0));
}

#[test]
fn ifna_on_na_returns_fallback() {
    assert_eq!(
        ifna_fn(&[Value::Error(ErrorKind::NA), Value::Text("n/a".to_string())]),
        Value::Text("n/a".to_string())
    );
}
