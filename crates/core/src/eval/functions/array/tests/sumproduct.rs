use super::super::sumproduct_fn;
use crate::types::{ErrorKind, Value};

fn num(n: f64) -> Value {
    Value::Number(n)
}

fn flat(ns: &[f64]) -> Value {
    Value::Array(ns.iter().map(|&n| num(n)).collect())
}

#[test]
fn sumproduct_two_arrays() {
    // SUMPRODUCT({1,2,3}, {4,5,6}) = 1*4 + 2*5 + 3*6 = 4+10+18 = 32
    let result = sumproduct_fn(&[flat(&[1.0, 2.0, 3.0]), flat(&[4.0, 5.0, 6.0])]);
    assert_eq!(result, num(32.0));
}

#[test]
fn sumproduct_single_array() {
    // SUMPRODUCT({2,3,4}) = 2+3+4 = 9
    let result = sumproduct_fn(&[flat(&[2.0, 3.0, 4.0])]);
    assert_eq!(result, num(9.0));
}

#[test]
fn sumproduct_mismatched_lengths() {
    let result = sumproduct_fn(&[flat(&[1.0, 2.0]), flat(&[1.0, 2.0, 3.0])]);
    assert_eq!(result, Value::Error(ErrorKind::Value));
}

#[test]
fn sumproduct_wrong_arity() {
    assert_eq!(sumproduct_fn(&[]), Value::Error(ErrorKind::NA));
}
