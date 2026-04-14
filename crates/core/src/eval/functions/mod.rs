pub mod logical;
pub mod math;

use std::collections::HashMap;
use crate::eval::context::Context;
use crate::parser::ast::Expr;
use crate::types::{ErrorKind, Value};

// ── EvalCtx ───────────────────────────────────────────────────────────────

/// Bundles evaluation context and function registry for use by Lazy functions.
pub struct EvalCtx<'r> {
    pub ctx: Context,
    pub registry: &'r Registry,
}

impl<'r> EvalCtx<'r> {
    pub fn new(ctx: Context, registry: &'r Registry) -> Self {
        Self { ctx, registry }
    }
}

// ── Function kinds ─────────────────────────────────────────────────────────

pub type EagerFn = fn(&[Value]) -> Value;
pub type LazyFn  = fn(&[Expr], &mut EvalCtx<'_>) -> Value;

pub enum FunctionKind {
    Eager(EagerFn),
    Lazy(LazyFn),
}

// ── Registry ──────────────────────────────────────────────────────────────

pub struct Registry {
    functions: HashMap<String, FunctionKind>,
}

impl Registry {
    pub fn new() -> Self {
        let mut r = Self { functions: HashMap::new() };
        math::register_math(&mut r);
        logical::register_logical(&mut r);
        r
    }

    pub fn register_eager(&mut self, name: &str, f: EagerFn) {
        self.functions.insert(name.to_uppercase(), FunctionKind::Eager(f));
    }

    pub fn register_lazy(&mut self, name: &str, f: LazyFn) {
        self.functions.insert(name.to_uppercase(), FunctionKind::Lazy(f));
    }

    pub fn get(&self, name: &str) -> Option<&FunctionKind> {
        self.functions.get(&name.to_uppercase())
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

/// Validate argument count. Returns `Some(Value::Error(ErrorKind::Value))` if out of range.
pub fn check_arity(args: &[Value], min: usize, max: usize) -> Option<Value> {
    if args.len() < min || args.len() > max {
        Some(Value::Error(ErrorKind::Value))
    } else {
        None
    }
}
