// truecalc-core: spreadsheet formula parser and evaluator

pub mod display;
pub mod engine;
pub mod eval;
pub mod parser;
pub mod types;

pub use display::display_number;
pub use engine::Engine;
pub use parser::{parse, validate, Expr};
pub use types::{ErrorKind, ParseError, Value};

pub use eval::functions::{FunctionMeta, Registry};

use std::collections::HashMap;

/// Evaluate a formula string with named variables, targeting Google Sheets conformance.
///
/// Returns `Value::Error(ErrorKind::Value)` on parse failure.
pub fn evaluate(formula: &str, variables: &HashMap<String, Value>) -> Value {
    Engine::google_sheets().evaluate(formula, variables)
}
