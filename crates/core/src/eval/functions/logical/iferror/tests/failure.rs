use super::super::{iferror_fn, ifna_fn};
use crate::types::{ErrorKind, Value};

#[test]
fn iferror_too_few_args() {
    assert_eq!(iferror_fn(&[Value::Number(1.0)]), Value::Error(ErrorKind::Value));
}

#[test]
fn iferror_too_many_args() {
    assert_eq!(
        iferror_fn(&[Value::Number(1.0), Value::Number(2.0), Value::Number(3.0)]),
        Value::Error(ErrorKind::Value)
    );
}

#[test]
fn ifna_too_few_args() {
    assert_eq!(ifna_fn(&[Value::Number(1.0)]), Value::Error(ErrorKind::Value));
}
