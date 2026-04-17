use super::super::array_constrain_fn;
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

#[test]
fn constrain_2x3_to_1x2() {
    // ARRAY_CONSTRAIN([[1,2,3],[4,5,6]], 1, 2) → [[1,2]]
    let arr = make_2d(&[&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]]);
    // from_2d with single row returns flat Array
    let expected = Value::Array(vec![num(1.0), num(2.0)]);
    assert_eq!(array_constrain_fn(&[arr, num(1.0), num(2.0)]), expected);
}

#[test]
fn constrain_exact_size() {
    // Constrain to same size → unchanged
    let arr = make_2d(&[&[1.0, 2.0], &[3.0, 4.0]]);
    let expected = make_2d(&[&[1.0, 2.0], &[3.0, 4.0]]);
    assert_eq!(array_constrain_fn(&[arr, num(2.0), num(2.0)]), expected);
}

#[test]
fn constrain_larger_than_array_uses_full() {
    // Requesting more rows/cols than available → returns full array
    let arr = make_2d(&[&[1.0, 2.0], &[3.0, 4.0]]);
    let expected = make_2d(&[&[1.0, 2.0], &[3.0, 4.0]]);
    assert_eq!(array_constrain_fn(&[arr, num(10.0), num(10.0)]), expected);
}

#[test]
fn constrain_invalid_zero_rows() {
    let arr = make_2d(&[&[1.0, 2.0]]);
    assert_eq!(
        array_constrain_fn(&[arr, num(0.0), num(1.0)]),
        Value::Error(ErrorKind::Value)
    );
}

#[test]
fn constrain_wrong_arity() {
    assert_eq!(array_constrain_fn(&[]), Value::Error(ErrorKind::NA));
    assert_eq!(
        array_constrain_fn(&[num(1.0), num(1.0)]),
        Value::Error(ErrorKind::NA)
    );
}
