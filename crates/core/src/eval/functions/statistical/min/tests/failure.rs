use super::super::*;
use crate::types::{ErrorKind, Value};

#[test]
fn min_ignores_error_values() {
    // The dispatcher strips errors before calling min_fn; errors are simply ignored
    assert_eq!(
        min_fn(&[Value::Number(3.0), Value::Error(ErrorKind::Value), Value::Number(5.0)]),
        Value::Number(3.0)
    );
}

#[test]
fn min_ignores_bool() {
    // Bool values are ignored; only Number(2) counts
    assert_eq!(
        min_fn(&[Value::Bool(true), Value::Number(2.0), Value::Bool(false)]),
        Value::Number(2.0)
    );
}
