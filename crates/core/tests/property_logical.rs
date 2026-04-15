use proptest::prelude::*;
use ganit_core::{evaluate, Value};
use std::collections::HashMap;

fn run(formula: &str) -> Value {
    evaluate(formula, &HashMap::new())
}

fn run_vars(formula: &str, vars: Vec<(&str, f64)>) -> Value {
    let map = vars.into_iter().map(|(k, v)| (k.to_string(), Value::Number(v))).collect();
    evaluate(formula, &map)
}

fn small_f64() -> impl Strategy<Value = f64> {
    -1e6f64..1e6f64
}

proptest! {
    // 1. IF(TRUE, a, b) == a
    #[test]
    fn if_true_returns_first(a in small_f64(), b in small_f64()) {
        let result = run_vars("=IF(TRUE, x, y)", vec![("x", a), ("y", b)]);
        prop_assert_eq!(result, Value::Number(a));
    }

    // 2. IF(FALSE, a, b) == b
    #[test]
    fn if_false_returns_second(a in small_f64(), b in small_f64()) {
        let result = run_vars("=IF(FALSE, x, y)", vec![("x", a), ("y", b)]);
        prop_assert_eq!(result, Value::Number(b));
    }

    // 3. ISNUMBER on a number is always TRUE
    #[test]
    fn isnumber_on_number_is_true(x in small_f64()) {
        let result = run_vars("=ISNUMBER(x)", vec![("x", x)]);
        prop_assert_eq!(result, Value::Bool(true));
    }

    // 4. ISTEXT on a number is always FALSE
    #[test]
    fn istext_on_number_is_false(x in small_f64()) {
        let result = run_vars("=ISTEXT(x)", vec![("x", x)]);
        prop_assert_eq!(result, Value::Bool(false));
    }

    // 5. IF selects the correct branch based on a comparison
    #[test]
    fn if_comparison_selects_correct_branch(a in small_f64(), b in small_f64()) {
        // IF(a < b, a, b) should be the minimum
        let result = run_vars("=IF(x < y, x, y)", vec![("x", a), ("y", b)]);
        let expected = if a < b { a } else { b };
        prop_assert_eq!(result, Value::Number(expected));
    }
}

// 6. NOT(NOT(TRUE)) == TRUE
#[test]
fn not_not_true() {
    assert_eq!(run("=NOT(NOT(TRUE))"), Value::Bool(true));
}

// 7. NOT(NOT(FALSE)) == FALSE
#[test]
fn not_not_false() {
    assert_eq!(run("=NOT(NOT(FALSE))"), Value::Bool(false));
}
