use super::super::transpose_fn;
use crate::types::{ErrorKind, Value};

fn num(n: f64) -> Value {
    Value::Number(n)
}

fn make_2d(rows: &[&[f64]]) -> Value {
    Value::Array(
        rows.iter()
            .map(|row| Value::Array(row.iter().map(|&n| Value::Number(n)).collect()))
            .collect(),
    )
}

#[test]
fn transpose_2x2() {
    // TRANSPOSE([[1,2],[3,4]]) → [[1,3],[2,4]]
    let input = make_2d(&[&[1.0, 2.0], &[3.0, 4.0]]);
    let expected = make_2d(&[&[1.0, 3.0], &[2.0, 4.0]]);
    assert_eq!(transpose_fn(&[input]), expected);
}

#[test]
fn transpose_2x3_to_3x2() {
    // TRANSPOSE([[1,2,3],[4,5,6]]) → [[1,4],[2,5],[3,6]]
    let input = make_2d(&[&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]]);
    let expected = make_2d(&[&[1.0, 4.0], &[2.0, 5.0], &[3.0, 6.0]]);
    assert_eq!(transpose_fn(&[input]), expected);
}

#[test]
fn transpose_1d_row_to_column() {
    // TRANSPOSE({1,2,3}) — flat 1D array (1 row, 3 cols) → 3 rows, 1 col each
    // from_2d with 3 single-element rows returns nested Array
    let input = Value::Array(vec![num(1.0), num(2.0), num(3.0)]);
    // Expected: 3 rows, each with 1 element → nested Array of single-element Arrays
    let expected = Value::Array(vec![
        Value::Array(vec![num(1.0)]),
        Value::Array(vec![num(2.0)]),
        Value::Array(vec![num(3.0)]),
    ]);
    assert_eq!(transpose_fn(&[input]), expected);
}

#[test]
fn transpose_scalar() {
    // TRANSPOSE(5) → 5 (1x1 stays 1x1, from_2d returns flat single-element array)
    // to_2d(scalar) → [[5]], transpose → [[5]], from_2d([[5]]) → Array([5])
    assert_eq!(transpose_fn(&[num(5.0)]), Value::Array(vec![num(5.0)]));
}

#[test]
fn transpose_wrong_arity() {
    assert_eq!(transpose_fn(&[]), Value::Error(ErrorKind::NA));
    assert_eq!(
        transpose_fn(&[num(1.0), num(2.0)]),
        Value::Error(ErrorKind::NA)
    );
}
