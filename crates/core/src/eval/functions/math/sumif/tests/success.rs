use super::super::sumif_fn;
use crate::types::Value;

fn nums(ns: &[f64]) -> Value {
    Value::Array(ns.iter().map(|&n| Value::Number(n)).collect())
}

fn texts(ss: &[&str]) -> Value {
    Value::Array(ss.iter().map(|s| Value::Text(s.to_string())).collect())
}

#[test]
fn sum_without_sum_range() {
    // SUMIF({1,2,3,4,5}, ">2") → 3+4+5 = 12
    let result = sumif_fn(&[nums(&[1.0, 2.0, 3.0, 4.0, 5.0]), Value::Text(">2".to_string())]);
    assert_eq!(result, Value::Number(12.0));
}

#[test]
fn sum_with_sum_range() {
    // SUMIF({"a","b","a","c"}, "a", {10,20,30,40}) → 10+30 = 40
    let result = sumif_fn(&[
        texts(&["a", "b", "a", "c"]),
        Value::Text("a".to_string()),
        nums(&[10.0, 20.0, 30.0, 40.0]),
    ]);
    assert_eq!(result, Value::Number(40.0));
}

#[test]
fn sum_exact_number_criterion() {
    // SUMIF({1,2,3,2,1}, 2, {10,20,30,20,10}) → 20+20 = 40
    let result = sumif_fn(&[
        nums(&[1.0, 2.0, 3.0, 2.0, 1.0]),
        Value::Number(2.0),
        nums(&[10.0, 20.0, 30.0, 20.0, 10.0]),
    ]);
    assert_eq!(result, Value::Number(40.0));
}

#[test]
fn sum_gt_criterion() {
    // SUMIF({1,2,3,4,5}, ">3") → 4+5 = 9
    let result = sumif_fn(&[nums(&[1.0, 2.0, 3.0, 4.0, 5.0]), Value::Text(">3".to_string())]);
    assert_eq!(result, Value::Number(9.0));
}

#[test]
fn sum_lte_criterion() {
    // SUMIF({1,2,3,4,5}, "<=2") → 1+2 = 3
    let result = sumif_fn(&[nums(&[1.0, 2.0, 3.0, 4.0, 5.0]), Value::Text("<=2".to_string())]);
    assert_eq!(result, Value::Number(3.0));
}

#[test]
fn sum_ne_criterion_with_sum_range() {
    // SUMIF({1,2,3}, "<>2", {10,20,30}) → 10+30 = 40
    let result = sumif_fn(&[
        nums(&[1.0, 2.0, 3.0]),
        Value::Text("<>2".to_string()),
        nums(&[10.0, 20.0, 30.0]),
    ]);
    assert_eq!(result, Value::Number(40.0));
}

#[test]
fn sum_wildcard_criterion() {
    // SUMIF({"apple","banana","apricot"}, "ap*", {10,20,30}) → 10+30 = 40
    let result = sumif_fn(&[
        texts(&["apple", "banana", "apricot"]),
        Value::Text("ap*".to_string()),
        nums(&[10.0, 20.0, 30.0]),
    ]);
    assert_eq!(result, Value::Number(40.0));
}

#[test]
fn sum_bool_in_sum_range() {
    // Booleans in sum_range: TRUE=1, FALSE=0
    let range = Value::Array(vec![Value::Number(1.0), Value::Number(2.0)]);
    let sum_range = Value::Array(vec![Value::Bool(true), Value::Bool(false)]);
    let result = sumif_fn(&[range, Value::Text(">=1".to_string()), sum_range]);
    assert_eq!(result, Value::Number(1.0)); // TRUE(1) + FALSE(0) = 1
}
