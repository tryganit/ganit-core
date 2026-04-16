use super::super::sumif_fn;
use crate::types::Value;

fn nums(ns: &[f64]) -> Value {
    Value::Array(ns.iter().map(|&n| Value::Number(n)).collect())
}

#[test]
fn no_matches_returns_zero() {
    // SUMIF({1,2,3}, ">10") → 0
    let result = sumif_fn(&[nums(&[1.0, 2.0, 3.0]), Value::Text(">10".to_string())]);
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn empty_array_returns_zero() {
    let result = sumif_fn(&[Value::Array(vec![]), Value::Number(1.0)]);
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn sum_range_shorter_than_range_uses_zip() {
    // range has 4 elements, sum_range has 2; only first 2 pairs are considered.
    // Criteria: ">0" matches all. Sum of sum_range = 10+20 = 30.
    let result = sumif_fn(&[
        nums(&[1.0, 2.0, 3.0, 4.0]),
        Value::Text(">0".to_string()),
        nums(&[10.0, 20.0]),
    ]);
    assert_eq!(result, Value::Number(30.0));
}

#[test]
fn non_numeric_text_in_sum_range_skipped() {
    // sum_range has non-numeric text for matched positions — should be skipped.
    let range = Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]);
    let sum_range = Value::Array(vec![
        Value::Text("abc".to_string()),
        Value::Number(5.0),
    ]);
    let result = sumif_fn(&[range, Value::Text(">=1".to_string()), sum_range]);
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn numeric_text_in_sum_range_is_summed() {
    // "3" as text in sum_range should parse and be added.
    let range = Value::Array(vec![Value::Number(1.0)]);
    let sum_range = Value::Array(vec![Value::Text("3".to_string())]);
    let result = sumif_fn(&[range, Value::Text("=1".to_string()), sum_range]);
    assert_eq!(result, Value::Number(3.0));
}

#[test]
fn scalar_range_matched() {
    // SUMIF(5, ">=5", 100) → 100
    let result = sumif_fn(&[Value::Number(5.0), Value::Text(">=5".to_string()), Value::Number(100.0)]);
    assert_eq!(result, Value::Number(100.0));
}
