pub mod context;
pub mod coercion;
pub mod functions;

pub use context::Context;
pub use functions::{EvalCtx, Registry};

use crate::parser::ast::Expr;
use crate::types::Value;

/// Placeholder — implemented in Task 7.
pub fn evaluate_expr(_expr: &Expr, _ctx: &mut EvalCtx<'_>) -> Value {
    todo!("implemented in Task 7")
}
