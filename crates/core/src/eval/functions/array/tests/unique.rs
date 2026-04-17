use super::super::unique_fn;
use crate::types::{ErrorKind, Value};

fn num(n: f64) -> Value {
    Value::Number(n)
}

// Column vector: each element as a single-element row Array.
// UNIQUE deduplicates rows, so a column vector is the natural 1D input.
fn col(ns: &[f64]) -> Value {
    Value::Array(
        ns.iter()
            .map(|&n| Value::Array(vec![num(n)]))
            .collect(),
    )
}

#[test]
fn unique_removes_duplicate_rows() {
    // UNIQUE([[1],[2],[2],[3],[1]]) → [[1],[2],[3]]
    let result = unique_fn(&[col(&[1.0, 2.0, 2.0, 3.0, 1.0])]);
    assert_eq!(result, col(&[1.0, 2.0, 3.0]));
}

#[test]
fn unique_no_duplicates() {
    // Already unique — unchanged
    let result = unique_fn(&[col(&[4.0, 5.0, 6.0])]);
    assert_eq!(result, col(&[4.0, 5.0, 6.0]));
}

#[test]
fn unique_all_same() {
    // UNIQUE([[7],[7],[7]]) → [[7]] — from_2d with 1 row returns flat Array
    let result = unique_fn(&[col(&[7.0, 7.0, 7.0])]);
    assert_eq!(result, Value::Array(vec![num(7.0)]));
}

#[test]
fn unique_exactly_once() {
    // UNIQUE([[1],[2],[2],[3]], false, true) → only rows appearing exactly once → [[1],[3]]
    let result = unique_fn(&[
        col(&[1.0, 2.0, 2.0, 3.0]),
        Value::Bool(false),
        Value::Bool(true),
    ]);
    assert_eq!(result, col(&[1.0, 3.0]));
}

#[test]
fn unique_2d_deduplicates_rows() {
    // UNIQUE([[1,2],[3,4],[1,2]]) → [[1,2],[3,4]]
    let input = Value::Array(vec![
        Value::Array(vec![num(1.0), num(2.0)]),
        Value::Array(vec![num(3.0), num(4.0)]),
        Value::Array(vec![num(1.0), num(2.0)]),
    ]);
    let expected = Value::Array(vec![
        Value::Array(vec![num(1.0), num(2.0)]),
        Value::Array(vec![num(3.0), num(4.0)]),
    ]);
    assert_eq!(unique_fn(&[input]), expected);
}

#[test]
fn unique_wrong_arity() {
    assert_eq!(unique_fn(&[]), Value::Error(ErrorKind::NA));
}
