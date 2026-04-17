use super::super::flatten_fn;
use crate::types::{ErrorKind, Value};

fn num(n: f64) -> Value {
    Value::Number(n)
}

fn make_2d(rows: &[&[f64]]) -> Value {
    Value::Array(
        rows.iter()
            .map(|row| Value::Array(row.iter().map(|&n| num(n)).collect()))
            .collect(),
    )
}

// FLATTEN returns a column vector: nested Array of single-element row Arrays
fn col(ns: &[f64]) -> Value {
    Value::Array(
        ns.iter()
            .map(|&n| Value::Array(vec![num(n)]))
            .collect(),
    )
}

#[test]
fn flatten_2d_array() {
    // FLATTEN([[1,2],[3,4]]) → column vector [1,2,3,4]
    let arr = make_2d(&[&[1.0, 2.0], &[3.0, 4.0]]);
    assert_eq!(flatten_fn(&[arr]), col(&[1.0, 2.0, 3.0, 4.0]));
}

#[test]
fn flatten_1d_array() {
    // FLATTEN({1,2,3}) → column vector [1,2,3]
    let arr = Value::Array(vec![num(1.0), num(2.0), num(3.0)]);
    assert_eq!(flatten_fn(&[arr]), col(&[1.0, 2.0, 3.0]));
}

#[test]
fn flatten_scalar() {
    // FLATTEN(5) → column vector [5] (single-element; from_2d with 1 row returns flat Array)
    let result = flatten_fn(&[num(5.0)]);
    // 1 item → from_2d returns flat single-element Array
    assert_eq!(result, Value::Array(vec![num(5.0)]));
}

#[test]
fn flatten_wrong_arity() {
    assert_eq!(flatten_fn(&[]), Value::Error(ErrorKind::NA));
    assert_eq!(flatten_fn(&[num(1.0), num(2.0)]), Value::Error(ErrorKind::NA));
}
