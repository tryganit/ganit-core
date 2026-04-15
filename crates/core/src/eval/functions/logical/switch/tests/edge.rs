use super::super::switch_fn;
use crate::eval::{Context, EvalCtx, Registry};
use crate::parser::ast::{Expr, Span};
use crate::types::Value;

fn span() -> Span { Span::new(0, 1) }

fn run(args: Vec<Expr>) -> Value {
    let reg = Registry::new();
    let mut ctx = EvalCtx::new(Context::empty(), &reg);
    switch_fn(&args, &mut ctx)
}

#[test]
fn bool_match() {
    let args = vec![
        Expr::Bool(true, span()),
        Expr::Bool(false, span()),
        Expr::Text("false".to_string(), span()),
        Expr::Bool(true, span()),
        Expr::Text("true".to_string(), span()),
    ];
    assert_eq!(run(args), Value::Text("true".to_string()));
}

#[test]
fn text_match() {
    let args = vec![
        Expr::Text("b".to_string(), span()),
        Expr::Text("a".to_string(), span()),
        Expr::Number(1.0, span()),
        Expr::Text("b".to_string(), span()),
        Expr::Number(2.0, span()),
    ];
    assert_eq!(run(args), Value::Number(2.0));
}

#[test]
fn single_case_with_default_no_match_uses_default() {
    // SWITCH(5, 1, "one", "default") — rest=[1,"one","default"], odd=true => default="default"
    let args = vec![
        Expr::Number(5.0, span()),
        Expr::Number(1.0, span()),
        Expr::Text("one".to_string(), span()),
        Expr::Text("default".to_string(), span()),
    ];
    assert_eq!(run(args), Value::Text("default".to_string()));
}
