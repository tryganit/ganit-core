use super::super::averageif_fn;
use crate::types::Value;

fn nums(ns: &[f64]) -> Value {
    Value::Array(ns.iter().map(|&n| Value::Number(n)).collect())
}

#[test]
fn avg_range_shorter_than_range_uses_zip() {
    // range has 4 elements, avg_range has 2; zip stops at 2.
    // ">0" matches all; only values 10 and 20 are averaged → 15.
    let result = averageif_fn(&[
        nums(&[1.0, 2.0, 3.0, 4.0]),
        Value::Text(">0".to_string()),
        nums(&[10.0, 20.0]),
    ]);
    assert_eq!(result, Value::Number(15.0));
}

#[test]
fn non_numeric_text_in_avg_range_skipped_from_count() {
    // avg_range: "abc" (skipped), 5.0 → only one matched numeric value → avg = 5.
    let range = Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]);
    let avg_range = Value::Array(vec![
        Value::Text("abc".to_string()),
        Value::Number(5.0),
    ]);
    let result = averageif_fn(&[range, Value::Text(">=1".to_string()), avg_range]);
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn numeric_text_in_avg_range_counts() {
    // "3" as text parses to 3.0 and is included in count.
    let range = Value::Array(vec![Value::Number(1.0), Value::Number(1.0)]);
    let avg_range = Value::Array(vec![Value::Text("4".to_string()), Value::Number(2.0)]);
    let result = averageif_fn(&[range, Value::Text("=1".to_string()), avg_range]);
    assert_eq!(result, Value::Number(3.0)); // (4+2)/2
}

#[test]
fn scalar_range_matched() {
    // AVERAGEIF(5, ">=5", 100) → 100
    let result = averageif_fn(&[
        Value::Number(5.0),
        Value::Text(">=5".to_string()),
        Value::Number(100.0),
    ]);
    assert_eq!(result, Value::Number(100.0));
}

#[test]
fn single_match_is_value_itself() {
    // Average of a single matched value is that value.
    let result = averageif_fn(&[nums(&[1.0, 7.0, 3.0]), Value::Text("=7".to_string())]);
    assert_eq!(result, Value::Number(7.0));
}
