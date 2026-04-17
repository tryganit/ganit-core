use super::super::{columns_fn, rows_fn};
use crate::types::{ErrorKind, Value};

fn make_2d(rows: &[&[f64]]) -> Value {
    Value::Array(
        rows.iter()
            .map(|row| Value::Array(row.iter().map(|&n| Value::Number(n)).collect()))
            .collect(),
    )
}

fn num(n: f64) -> Value {
    Value::Number(n)
}

// ── ROWS ─────────────────────────────────────────────────────────────────────

#[test]
fn rows_2x3_array() {
    let arr = make_2d(&[&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]]);
    assert_eq!(rows_fn(&[arr]), num(2.0));
}

#[test]
fn rows_scalar() {
    assert_eq!(rows_fn(&[num(42.0)]), num(1.0));
}

#[test]
fn rows_1d_flat_array() {
    // Flat array (no nested Arrays) → one row
    let arr = Value::Array(vec![num(1.0), num(2.0), num(3.0)]);
    assert_eq!(rows_fn(&[arr]), num(1.0));
}

#[test]
fn rows_wrong_arity() {
    assert_eq!(rows_fn(&[]), Value::Error(ErrorKind::NA));
    assert_eq!(rows_fn(&[num(1.0), num(2.0)]), Value::Error(ErrorKind::NA));
}

// ── COLUMNS ───────────────────────────────────────────────────────────────────

#[test]
fn columns_2x3_array() {
    let arr = make_2d(&[&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]]);
    assert_eq!(columns_fn(&[arr]), num(3.0));
}

#[test]
fn columns_scalar() {
    assert_eq!(columns_fn(&[num(42.0)]), num(1.0));
}

#[test]
fn columns_1d_flat_array() {
    // Flat array is one row → 3 columns
    let arr = Value::Array(vec![num(1.0), num(2.0), num(3.0)]);
    assert_eq!(columns_fn(&[arr]), num(3.0));
}

#[test]
fn columns_wrong_arity() {
    assert_eq!(columns_fn(&[]), Value::Error(ErrorKind::NA));
    assert_eq!(
        columns_fn(&[num(1.0), num(2.0)]),
        Value::Error(ErrorKind::NA)
    );
}
