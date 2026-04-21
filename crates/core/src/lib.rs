// truecalc-core: spreadsheet formula parser and evaluator

pub mod display;
pub mod eval;
pub mod parser;
pub mod types;

pub use display::display_number;
pub use parser::{parse, validate, Expr};
pub use types::{ErrorKind, ParseError, Value};

pub use eval::functions::{FunctionMeta, Registry};

use std::collections::HashMap;
use eval::{evaluate_expr, Context, EvalCtx};

/// Evaluate a formula string with named variables.
///
/// Returns `Value::Error(ErrorKind::Value)` on parse failure.
///
/// Note: constructs a fresh `Registry` on every call. For bulk evaluation
/// across many formulas, consider reusing a `Registry` via the lower-level
/// `eval::evaluate_expr` API.
pub fn evaluate(formula: &str, variables: &HashMap<String, Value>) -> Value {
    match parse(formula) {
        Err(_) => Value::Error(ErrorKind::Value),
        Ok(expr) => {
            let ctx = Context::new(variables.clone());
            let registry = Registry::new();
            let mut eval_ctx = EvalCtx::new(ctx, &registry);
            first_of_array(evaluate_expr(&expr, &mut eval_ctx))
        }
    }
}

/// In Google Sheets, placing an array-returning formula in a single cell yields
/// the first (top-left) element. This helper replicates that scalar-context
/// unwrapping: 1-D `[x, y, …]` → `x`, 2-D `[[a, b], …]` → `a`, scalars pass through.
fn first_of_array(v: Value) -> Value {
    match v {
        Value::Array(elems) if !elems.is_empty() => {
            first_of_array(elems.into_iter().next().unwrap())
        }
        other => other,
    }
}
