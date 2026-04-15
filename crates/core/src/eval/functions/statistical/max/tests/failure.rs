use super::super::*;
use crate::types::{ErrorKind, Value};

#[test]
fn max_ignores_error_values() {
    // The dispatcher strips errors before calling max_fn; errors are simply ignored
    assert_eq!(
        max_fn(&[Value::Number(3.0), Value::Error(ErrorKind::Value), Value::Number(5.0)]),
        Value::Number(5.0)
    );
}

#[test]
fn max_ignores_bool() {
    // Bool values are ignored; only Number(10) counts
    assert_eq!(
        max_fn(&[Value::Bool(true), Value::Number(10.0), Value::Bool(false)]),
        Value::Number(10.0)
    );
}
