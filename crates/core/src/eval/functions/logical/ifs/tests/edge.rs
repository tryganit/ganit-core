use super::super::ifs_fn;
use crate::eval::{Context, EvalCtx, Registry};
use crate::parser::ast::{Expr, Span};
use crate::types::{ErrorKind, Value};

fn span() -> Span { Span::new(0, 1) }

fn run(args: Vec<Expr>) -> Value {
    let reg = Registry::new();
    let mut ctx = EvalCtx::new(Context::empty(), &reg);
    ifs_fn(&args, &mut ctx)
}

#[test]
fn single_pair_false_returns_na() {
    let args = vec![Expr::Bool(false, span()), Expr::Number(1.0, span())];
    assert_eq!(run(args), Value::Error(ErrorKind::NA));
}

#[test]
fn condition_coercion_error_propagates() {
    let args = vec![
        Expr::Text("bad".to_string(), span()),
        Expr::Number(1.0, span()),
    ];
    assert_eq!(run(args), Value::Error(ErrorKind::Value));
}

#[test]
fn number_zero_condition_is_falsy() {
    let args = vec![
        Expr::Number(0.0, span()),
        Expr::Number(1.0, span()),
        Expr::Bool(true, span()),
        Expr::Number(2.0, span()),
    ];
    assert_eq!(run(args), Value::Number(2.0));
}
