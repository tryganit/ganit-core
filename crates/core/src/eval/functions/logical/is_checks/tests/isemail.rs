use super::super::is_valid_email;
use crate::evaluate;
use crate::types::{ErrorKind, Value};
use std::collections::HashMap;

fn run(formula: &str) -> Value {
    evaluate(formula, &HashMap::new())
}

// ── is_valid_email helper ─────────────────────────────────────────────────────

#[test]
fn valid_simple_email() {
    assert!(is_valid_email("user@example.com"));
}

#[test]
fn valid_with_subdomain() {
    assert!(is_valid_email("user@mail.example.com"));
}

#[test]
fn valid_with_plus_tag() {
    assert!(is_valid_email("user+tag@example.com"));
}

#[test]
fn missing_at_sign() {
    assert!(!is_valid_email("notanemail"));
}

#[test]
fn missing_domain() {
    assert!(!is_valid_email("user@"));
}

#[test]
fn missing_local_part() {
    assert!(!is_valid_email("@example.com"));
}

#[test]
fn no_dot_in_domain() {
    assert!(!is_valid_email("user@localhost"));
}

// ── ISEMAIL function ──────────────────────────────────────────────────────────

#[test]
fn number_arg_returns_false() {
    assert_eq!(run("=ISEMAIL(42)"), Value::Bool(false));
}

#[test]
fn bool_arg_returns_false() {
    assert_eq!(run("=ISEMAIL(TRUE)"), Value::Bool(false));
}

#[test]
fn zero_args_returns_na() {
    assert_eq!(run("=ISEMAIL()"), Value::Error(ErrorKind::NA));
}
