use super::super::{iferror_fn, ifna_fn};
use crate::types::{ErrorKind, Value};

/// IFERROR catches ALL error kinds including NA.
#[test]
fn iferror_catches_na() {
    assert_eq!(
        iferror_fn(&[Value::Error(ErrorKind::NA), Value::Bool(false)]),
        Value::Bool(false)
    );
}

/// IFNA does NOT catch non-NA errors — it passes them through.
#[test]
fn ifna_passes_through_non_na_error() {
    assert_eq!(
        ifna_fn(&[Value::Error(ErrorKind::DivByZero), Value::Number(0.0)]),
        Value::Error(ErrorKind::DivByZero)
    );
}

/// IFNA does NOT catch Value errors.
#[test]
fn ifna_passes_through_value_error() {
    assert_eq!(
        ifna_fn(&[Value::Error(ErrorKind::Value), Value::Number(0.0)]),
        Value::Error(ErrorKind::Value)
    );
}
