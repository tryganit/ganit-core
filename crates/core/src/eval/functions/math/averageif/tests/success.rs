use super::super::averageif_fn;
use crate::types::Value;

fn nums(ns: &[f64]) -> Value {
    Value::Array(ns.iter().map(|&n| Value::Number(n)).collect())
}

fn texts(ss: &[&str]) -> Value {
    Value::Array(ss.iter().map(|s| Value::Text(s.to_string())).collect())
}

#[test]
fn average_without_avg_range() {
    // AVERAGEIF({1,2,3,4,5}, ">2") → avg(3,4,5) = 4
    let result = averageif_fn(&[nums(&[1.0, 2.0, 3.0, 4.0, 5.0]), Value::Text(">2".to_string())]);
    assert_eq!(result, Value::Number(4.0));
}

#[test]
fn average_with_avg_range() {
    // AVERAGEIF({"a","b","a","c"}, "a", {10,20,30,40}) → avg(10,30) = 20
    let result = averageif_fn(&[
        texts(&["a", "b", "a", "c"]),
        Value::Text("a".to_string()),
        nums(&[10.0, 20.0, 30.0, 40.0]),
    ]);
    assert_eq!(result, Value::Number(20.0));
}

#[test]
fn average_exact_number_criterion() {
    // AVERAGEIF({1,2,3,2,1}, 2, {10,20,30,40,50}) → avg(20,40) = 30
    let result = averageif_fn(&[
        nums(&[1.0, 2.0, 3.0, 2.0, 1.0]),
        Value::Number(2.0),
        nums(&[10.0, 20.0, 30.0, 40.0, 50.0]),
    ]);
    assert_eq!(result, Value::Number(30.0));
}

#[test]
fn average_gte_criterion() {
    // AVERAGEIF({1,2,3,4,5}, ">=4") → avg(4,5) = 4.5
    let result = averageif_fn(&[nums(&[1.0, 2.0, 3.0, 4.0, 5.0]), Value::Text(">=4".to_string())]);
    assert_eq!(result, Value::Number(4.5));
}

#[test]
fn average_lte_criterion() {
    // AVERAGEIF({1,2,3,4,5}, "<=2") → avg(1,2) = 1.5
    let result = averageif_fn(&[nums(&[1.0, 2.0, 3.0, 4.0, 5.0]), Value::Text("<=2".to_string())]);
    assert_eq!(result, Value::Number(1.5));
}

#[test]
fn average_ne_criterion_with_avg_range() {
    // AVERAGEIF({1,2,3}, "<>2", {10,20,30}) → avg(10,30) = 20
    let result = averageif_fn(&[
        nums(&[1.0, 2.0, 3.0]),
        Value::Text("<>2".to_string()),
        nums(&[10.0, 20.0, 30.0]),
    ]);
    assert_eq!(result, Value::Number(20.0));
}

#[test]
fn average_wildcard_criterion() {
    // AVERAGEIF({"apple","banana","apricot"}, "ap*", {10,20,30}) → avg(10,30) = 20
    let result = averageif_fn(&[
        texts(&["apple", "banana", "apricot"]),
        Value::Text("ap*".to_string()),
        nums(&[10.0, 20.0, 30.0]),
    ]);
    assert_eq!(result, Value::Number(20.0));
}

#[test]
fn average_bool_in_avg_range() {
    // TRUE=1, FALSE=0; avg(1,0) = 0.5
    let range = Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]);
    let avg_range = Value::Array(vec![Value::Bool(true), Value::Bool(false)]);
    let result = averageif_fn(&[range, Value::Text(">=1".to_string()), avg_range]);
    assert_eq!(result, Value::Number(0.5));
}
