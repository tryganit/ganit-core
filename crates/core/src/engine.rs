use std::collections::HashMap;

use crate::eval::{evaluate_expr, Context, EvalCtx};
use crate::eval::functions::Registry;
use crate::parser::parse;
use crate::types::{ErrorKind, Value};

pub struct Engine {
    registry: Registry,
}

impl Engine {
    pub fn google_sheets() -> Self {
        Self { registry: Registry::new() }
    }

    pub fn evaluate(&self, formula: &str, variables: &HashMap<String, Value>) -> Value {
        match parse(formula) {
            Err(_) => Value::Error(ErrorKind::Value),
            Ok(expr) => {
                let ctx = Context::new(variables.clone());
                let mut eval_ctx = EvalCtx::new(ctx, &self.registry);
                first_of_array(evaluate_expr(&expr, &mut eval_ctx))
            }
        }
    }
}

fn first_of_array(v: Value) -> Value {
    match v {
        Value::Array(elems) if !elems.is_empty() => {
            first_of_array(elems.into_iter().next().unwrap())
        }
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn google_sheets_evaluates_sum() {
        let engine = Engine::google_sheets();
        let result = engine.evaluate("=SUM(1,2)", &HashMap::new());
        assert_eq!(result, Value::Number(3.0));
    }

    #[test]
    fn google_sheets_evaluates_with_variables() {
        let engine = Engine::google_sheets();
        let mut vars = HashMap::new();
        vars.insert("A".to_string(), Value::Number(10.0));
        let result = engine.evaluate("=A+5", &vars);
        assert_eq!(result, Value::Number(15.0));
    }

    #[test]
    fn parse_error_returns_value_error() {
        let engine = Engine::google_sheets();
        let result = engine.evaluate("not a formula", &HashMap::new());
        assert_eq!(result, Value::Error(ErrorKind::Value));
    }
}
