use super::super::sort_fn;
use crate::types::{ErrorKind, Value};

fn num(n: f64) -> Value {
    Value::Number(n)
}

// A column vector: each element is its own single-element row Array.
// SORT operates on rows, so a column vector (N rows × 1 col) is the natural 1D input.
fn col(ns: &[f64]) -> Value {
    Value::Array(
        ns.iter()
            .map(|&n| Value::Array(vec![num(n)]))
            .collect(),
    )
}

fn make_2d(rows: &[&[f64]]) -> Value {
    Value::Array(
        rows.iter()
            .map(|row| Value::Array(row.iter().map(|&n| num(n)).collect()))
            .collect(),
    )
}

#[test]
fn sort_column_vector_ascending() {
    // SORT([[3],[1],[2]], 1, 1) → [[1],[2],[3]]
    let result = sort_fn(&[col(&[3.0, 1.0, 2.0]), num(1.0), num(1.0)]);
    assert_eq!(result, col(&[1.0, 2.0, 3.0]));
}

#[test]
fn sort_column_vector_ascending_default() {
    // SORT([[3],[1],[2]]) — default ascending
    let result = sort_fn(&[col(&[3.0, 1.0, 2.0])]);
    assert_eq!(result, col(&[1.0, 2.0, 3.0]));
}

#[test]
fn sort_column_vector_descending() {
    // SORT([[3],[1],[2]], 1, -1) → [[3],[2],[1]]
    let result = sort_fn(&[col(&[3.0, 1.0, 2.0]), num(1.0), num(-1.0)]);
    assert_eq!(result, col(&[3.0, 2.0, 1.0]));
}

#[test]
fn sort_2d_by_first_col_ascending() {
    // SORT([[3,a],[1,b],[2,c]], 1, 1) → [[1,b],[2,c],[3,a]]
    let input = Value::Array(vec![
        Value::Array(vec![num(3.0), Value::Text("a".to_string())]),
        Value::Array(vec![num(1.0), Value::Text("b".to_string())]),
        Value::Array(vec![num(2.0), Value::Text("c".to_string())]),
    ]);
    let expected = Value::Array(vec![
        Value::Array(vec![num(1.0), Value::Text("b".to_string())]),
        Value::Array(vec![num(2.0), Value::Text("c".to_string())]),
        Value::Array(vec![num(3.0), Value::Text("a".to_string())]),
    ]);
    assert_eq!(sort_fn(&[input, num(1.0), num(1.0)]), expected);
}

#[test]
fn sort_already_sorted() {
    let result = sort_fn(&[col(&[1.0, 2.0, 3.0])]);
    assert_eq!(result, col(&[1.0, 2.0, 3.0]));
}

#[test]
fn sort_single_element() {
    // Single-row 2D array stays the same; from_2d with 1 row returns flat Array
    let single = make_2d(&[&[7.0]]);
    let expected = Value::Array(vec![num(7.0)]);
    assert_eq!(sort_fn(&[single]), expected);
}

#[test]
fn sort_wrong_arity() {
    assert_eq!(sort_fn(&[]), Value::Error(ErrorKind::NA));
}
