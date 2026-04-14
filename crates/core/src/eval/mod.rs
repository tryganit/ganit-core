pub mod context;
pub mod coercion;
pub mod functions;

pub use context::Context;
pub use functions::{EvalCtx, Registry};

use crate::parser::ast::{BinaryOp, Expr, UnaryOp};
use crate::types::{ErrorKind, Value};

use coercion::{to_number, to_string_val};
use functions::FunctionKind;

/// Evaluate an expression tree, resolving variables from `ctx.ctx` and calling
/// functions from `ctx.registry`.
pub fn evaluate_expr(expr: &Expr, ctx: &mut EvalCtx<'_>) -> Value {
    match expr {
        // ── Leaf nodes ──────────────────────────────────────────────────────
        Expr::Number(n, _) => {
            if n.is_finite() {
                Value::Number(*n)
            } else {
                Value::Error(ErrorKind::Num)
            }
        }
        Expr::Text(s, _)   => Value::Text(s.clone()),
        Expr::Bool(b, _)   => Value::Bool(*b),
        Expr::Variable(name, _) => ctx.ctx.get(name),

        // ── Unary ops ───────────────────────────────────────────────────────
        Expr::UnaryOp { op, operand, .. } => {
            let val = evaluate_expr(operand, ctx);
            match to_number(val) {
                Err(e) => e,
                Ok(n)  => match op {
                    UnaryOp::Neg     => Value::Number(-n),
                    UnaryOp::Percent => Value::Number(n / 100.0),
                },
            }
        }

        // ── Binary ops ──────────────────────────────────────────────────────
        Expr::BinaryOp { op, left, right, .. } => {
            let lv = evaluate_expr(left, ctx);
            let rv = evaluate_expr(right, ctx);
            eval_binary(op, lv, rv)
        }

        // ── Function calls ──────────────────────────────────────────────────
        Expr::FunctionCall { name, args, .. } => {
            match ctx.registry.get(name) {
                None => Value::Error(ErrorKind::Name),
                Some(FunctionKind::Lazy(f)) => {
                    // Copy the fn pointer out to avoid holding a borrow on ctx.registry
                    // while also mutably borrowing ctx itself.
                    let f: functions::LazyFn = *f;
                    f(args, ctx)
                }
                Some(FunctionKind::Eager(f)) => {
                    let f: functions::EagerFn = *f;
                    // Evaluate all args; return first error encountered.
                    let mut evaluated = Vec::with_capacity(args.len());
                    for arg in args {
                        let v = evaluate_expr(arg, ctx);
                        if matches!(v, Value::Error(_)) {
                            return v;
                        }
                        evaluated.push(v);
                    }
                    f(&evaluated)
                }
            }
        }
    }
}

// ── Type ordering for cross-type comparisons (Excel semantics) ───────────────
// Number < Text < Bool  (Empty counts as Number)
fn type_rank(v: &Value) -> u8 {
    match v {
        Value::Number(_) | Value::Empty => 0,
        Value::Text(_)                  => 1,
        Value::Bool(_)                  => 2,
        // Error and Array cannot reach compare_values through the normal eval path
        // (eval_binary guards against errors before calling compare_values).
        Value::Error(_) | Value::Array(_) => 3,
    }
}

fn eval_binary(op: &BinaryOp, lv: Value, rv: Value) -> Value {
    match op {
        // ── Arithmetic ──────────────────────────────────────────────────────
        BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Pow => {
            let ln = match to_number(lv) { Ok(n) => n, Err(e) => return e };
            let rn = match to_number(rv) { Ok(n) => n, Err(e) => return e };
            let result = match op {
                BinaryOp::Add => ln + rn,
                BinaryOp::Sub => ln - rn,
                BinaryOp::Mul => ln * rn,
                BinaryOp::Div => {
                    if rn == 0.0 {
                        return Value::Error(ErrorKind::DivByZero);
                    }
                    ln / rn
                }
                BinaryOp::Pow => ln.powf(rn),
                // Safety: outer match arm covers exactly Add|Sub|Mul|Div|Pow; Concat and comparison ops are handled separately.
                _ => unreachable!(),
            };
            if !result.is_finite() {
                return Value::Error(ErrorKind::Num);
            }
            Value::Number(result)
        }

        // ── Concatenation ───────────────────────────────────────────────────
        BinaryOp::Concat => {
            let ls = match to_string_val(lv) { Ok(s) => s, Err(e) => return e };
            let rs = match to_string_val(rv) { Ok(s) => s, Err(e) => return e };
            Value::Text(ls + &rs)
        }

        // ── Comparisons ─────────────────────────────────────────────────────
        BinaryOp::Eq | BinaryOp::Ne
        | BinaryOp::Lt | BinaryOp::Gt
        | BinaryOp::Le | BinaryOp::Ge => {
            // Error propagation: left side first.
            if let Value::Error(_) = &lv { return lv; }
            if let Value::Error(_) = &rv { return rv; }

            let result = compare_values(op, &lv, &rv);
            Value::Bool(result)
        }
    }
}

/// Compare two (non-error) values with Excel ordering semantics.
fn compare_values(op: &BinaryOp, lv: &Value, rv: &Value) -> bool {
    match (lv, rv) {
        (Value::Number(a), Value::Number(b)) => apply_cmp(op, a.partial_cmp(b)),
        (Value::Text(a),   Value::Text(b))   => apply_cmp(op, Some(a.cmp(b))),
        (Value::Bool(a),   Value::Bool(b))   => apply_cmp(op, Some(a.cmp(b))),
        (Value::Empty,     Value::Empty)     => apply_cmp(op, Some(std::cmp::Ordering::Equal)),
        // Empty acts as Number(0)
        (Value::Empty, Value::Number(b))     => apply_cmp(op, 0.0f64.partial_cmp(b)),
        (Value::Number(a), Value::Empty)     => apply_cmp(op, a.partial_cmp(&0.0f64)),
        // Cross-type: use type rank
        _ => {
            let lr = type_rank(lv);
            let rr = type_rank(rv);
            match op {
                BinaryOp::Eq => false,
                BinaryOp::Ne => true,
                BinaryOp::Lt => lr < rr,
                BinaryOp::Gt => lr > rr,
                BinaryOp::Le => lr <= rr,
                BinaryOp::Ge => lr >= rr,
                // Safety: outer match arm covers exactly Eq|Ne|Lt|Gt|Le|Ge; arithmetic and Concat ops are handled separately.
                _ => unreachable!(),
            }
        }
    }
}

fn apply_cmp(op: &BinaryOp, ord: Option<std::cmp::Ordering>) -> bool {
    match ord {
        // NaN: per Value::Number invariant this should not occur after the is_finite() guard;
        // returning false matches Excel semantics if it somehow does.
        None => false,
        Some(o) => match op {
            BinaryOp::Eq => o.is_eq(),
            BinaryOp::Ne => o.is_ne(),
            BinaryOp::Lt => o.is_lt(),
            BinaryOp::Gt => o.is_gt(),
            BinaryOp::Le => o.is_le(),
            BinaryOp::Ge => o.is_ge(),
            // Safety: apply_cmp is only called from compare_values which is only called from eval_binary's comparison arm (Eq|Ne|Lt|Gt|Le|Ge).
            _ => unreachable!(),
        },
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::eval::{Context, EvalCtx, Registry};
    use crate::parser::ast::{BinaryOp, Expr, Span, UnaryOp};
    use crate::types::{ErrorKind, Value};

    fn dummy_span() -> Span { Span::new(0, 1) }

    fn make_ctx() -> (Registry, Context) {
        (Registry::new(), Context::empty())
    }

    fn run(expr: &Expr, registry: &Registry, ctx: Context) -> Value {
        let mut eval_ctx = EvalCtx::new(ctx, registry);
        evaluate_expr(expr, &mut eval_ctx)
    }

    // ── Leaf nodes ───────────────────────────────────────────────────────────

    #[test]
    fn leaf_number() {
        let (reg, ctx) = make_ctx();
        let expr = Expr::Number(42.0, dummy_span());
        assert_eq!(run(&expr, &reg, ctx), Value::Number(42.0));
    }

    #[test]
    fn leaf_text() {
        let (reg, ctx) = make_ctx();
        let expr = Expr::Text("hello".to_string(), dummy_span());
        assert_eq!(run(&expr, &reg, ctx), Value::Text("hello".to_string()));
    }

    #[test]
    fn leaf_bool_true() {
        let (reg, ctx) = make_ctx();
        let expr = Expr::Bool(true, dummy_span());
        assert_eq!(run(&expr, &reg, ctx), Value::Bool(true));
    }

    #[test]
    fn leaf_bool_false() {
        let (reg, ctx) = make_ctx();
        let expr = Expr::Bool(false, dummy_span());
        assert_eq!(run(&expr, &reg, ctx), Value::Bool(false));
    }

    #[test]
    fn variable_found() {
        let (reg, _) = make_ctx();
        let mut vars = std::collections::HashMap::new();
        vars.insert("x".to_string(), Value::Number(7.0));
        let ctx = Context::new(vars);
        let expr = Expr::Variable("x".to_string(), dummy_span());
        assert_eq!(run(&expr, &reg, ctx), Value::Number(7.0));
    }

    #[test]
    fn variable_not_found() {
        let (reg, ctx) = make_ctx();
        let expr = Expr::Variable("missing".to_string(), dummy_span());
        assert_eq!(run(&expr, &reg, ctx), Value::Empty);
    }

    // ── Unary ops ────────────────────────────────────────────────────────────

    #[test]
    fn unary_neg_number() {
        let (reg, ctx) = make_ctx();
        let expr = Expr::UnaryOp {
            op: UnaryOp::Neg,
            operand: Box::new(Expr::Number(5.0, dummy_span())),
            span: dummy_span(),
        };
        assert_eq!(run(&expr, &reg, ctx), Value::Number(-5.0));
    }

    #[test]
    fn unary_neg_text_error() {
        let (reg, ctx) = make_ctx();
        let expr = Expr::UnaryOp {
            op: UnaryOp::Neg,
            operand: Box::new(Expr::Text("abc".to_string(), dummy_span())),
            span: dummy_span(),
        };
        assert_eq!(run(&expr, &reg, ctx), Value::Error(ErrorKind::Value));
    }

    #[test]
    fn unary_percent() {
        let (reg, ctx) = make_ctx();
        let expr = Expr::UnaryOp {
            op: UnaryOp::Percent,
            operand: Box::new(Expr::Number(50.0, dummy_span())),
            span: dummy_span(),
        };
        assert_eq!(run(&expr, &reg, ctx), Value::Number(0.5));
    }

    // ── Binary arithmetic ────────────────────────────────────────────────────

    #[test]
    fn binary_add() {
        let (reg, ctx) = make_ctx();
        let expr = Expr::BinaryOp {
            op: BinaryOp::Add,
            left: Box::new(Expr::Number(3.0, dummy_span())),
            right: Box::new(Expr::Number(4.0, dummy_span())),
            span: dummy_span(),
        };
        assert_eq!(run(&expr, &reg, ctx), Value::Number(7.0));
    }

    #[test]
    fn binary_sub() {
        let (reg, ctx) = make_ctx();
        let expr = Expr::BinaryOp {
            op: BinaryOp::Sub,
            left: Box::new(Expr::Number(10.0, dummy_span())),
            right: Box::new(Expr::Number(3.0, dummy_span())),
            span: dummy_span(),
        };
        assert_eq!(run(&expr, &reg, ctx), Value::Number(7.0));
    }

    #[test]
    fn binary_mul() {
        let (reg, ctx) = make_ctx();
        let expr = Expr::BinaryOp {
            op: BinaryOp::Mul,
            left: Box::new(Expr::Number(6.0, dummy_span())),
            right: Box::new(Expr::Number(7.0, dummy_span())),
            span: dummy_span(),
        };
        assert_eq!(run(&expr, &reg, ctx), Value::Number(42.0));
    }

    #[test]
    fn binary_div() {
        let (reg, ctx) = make_ctx();
        let expr = Expr::BinaryOp {
            op: BinaryOp::Div,
            left: Box::new(Expr::Number(10.0, dummy_span())),
            right: Box::new(Expr::Number(4.0, dummy_span())),
            span: dummy_span(),
        };
        assert_eq!(run(&expr, &reg, ctx), Value::Number(2.5));
    }

    #[test]
    fn binary_div_by_zero() {
        let (reg, ctx) = make_ctx();
        let expr = Expr::BinaryOp {
            op: BinaryOp::Div,
            left: Box::new(Expr::Number(10.0, dummy_span())),
            right: Box::new(Expr::Number(0.0, dummy_span())),
            span: dummy_span(),
        };
        assert_eq!(run(&expr, &reg, ctx), Value::Error(ErrorKind::DivByZero));
    }

    #[test]
    fn binary_pow() {
        let (reg, ctx) = make_ctx();
        let expr = Expr::BinaryOp {
            op: BinaryOp::Pow,
            left: Box::new(Expr::Number(2.0, dummy_span())),
            right: Box::new(Expr::Number(10.0, dummy_span())),
            span: dummy_span(),
        };
        assert_eq!(run(&expr, &reg, ctx), Value::Number(1024.0));
    }

    // ── Concat ───────────────────────────────────────────────────────────────

    #[test]
    fn binary_concat() {
        let (reg, ctx) = make_ctx();
        let expr = Expr::BinaryOp {
            op: BinaryOp::Concat,
            left: Box::new(Expr::Text("hello".to_string(), dummy_span())),
            right: Box::new(Expr::Text(" world".to_string(), dummy_span())),
            span: dummy_span(),
        };
        assert_eq!(run(&expr, &reg, ctx), Value::Text("hello world".to_string()));
    }

    // ── Comparisons ──────────────────────────────────────────────────────────

    #[test]
    fn cmp_eq_same_type_equal() {
        let (reg, ctx) = make_ctx();
        let expr = Expr::BinaryOp {
            op: BinaryOp::Eq,
            left: Box::new(Expr::Number(5.0, dummy_span())),
            right: Box::new(Expr::Number(5.0, dummy_span())),
            span: dummy_span(),
        };
        assert_eq!(run(&expr, &reg, ctx), Value::Bool(true));
    }

    #[test]
    fn cmp_eq_same_type_unequal() {
        let (reg, ctx) = make_ctx();
        let expr = Expr::BinaryOp {
            op: BinaryOp::Eq,
            left: Box::new(Expr::Number(5.0, dummy_span())),
            right: Box::new(Expr::Number(6.0, dummy_span())),
            span: dummy_span(),
        };
        assert_eq!(run(&expr, &reg, ctx), Value::Bool(false));
    }

    #[test]
    fn cmp_eq_cross_type() {
        // Number vs Text: always false for Eq
        let (reg, ctx) = make_ctx();
        let expr = Expr::BinaryOp {
            op: BinaryOp::Eq,
            left: Box::new(Expr::Number(1.0, dummy_span())),
            right: Box::new(Expr::Text("1".to_string(), dummy_span())),
            span: dummy_span(),
        };
        assert_eq!(run(&expr, &reg, ctx), Value::Bool(false));
    }

    #[test]
    fn cmp_ne_cross_type() {
        let (reg, ctx) = make_ctx();
        let expr = Expr::BinaryOp {
            op: BinaryOp::Ne,
            left: Box::new(Expr::Number(1.0, dummy_span())),
            right: Box::new(Expr::Text("1".to_string(), dummy_span())),
            span: dummy_span(),
        };
        assert_eq!(run(&expr, &reg, ctx), Value::Bool(true));
    }

    #[test]
    fn cmp_lt_number_vs_text() {
        // Number < Text in Excel ordering
        let (reg, ctx) = make_ctx();
        let expr = Expr::BinaryOp {
            op: BinaryOp::Lt,
            left: Box::new(Expr::Number(999.0, dummy_span())),
            right: Box::new(Expr::Text("a".to_string(), dummy_span())),
            span: dummy_span(),
        };
        assert_eq!(run(&expr, &reg, ctx), Value::Bool(true));
    }

    #[test]
    fn cmp_bool_ordering() {
        // FALSE < TRUE
        let (reg, ctx) = make_ctx();
        let expr = Expr::BinaryOp {
            op: BinaryOp::Lt,
            left: Box::new(Expr::Bool(false, dummy_span())),
            right: Box::new(Expr::Bool(true, dummy_span())),
            span: dummy_span(),
        };
        assert_eq!(run(&expr, &reg, ctx), Value::Bool(true));
    }

    #[test]
    fn cmp_text_gt_number() {
        // Text > Number in Excel ordering
        let (reg, ctx) = make_ctx();
        let expr = Expr::BinaryOp {
            op: BinaryOp::Gt,
            left: Box::new(Expr::Text("a".to_string(), dummy_span())),
            right: Box::new(Expr::Number(999.0, dummy_span())),
            span: dummy_span(),
        };
        assert_eq!(run(&expr, &reg, ctx), Value::Bool(true));
    }

    // ── Function calls ───────────────────────────────────────────────────────

    #[test]
    fn function_unknown_returns_name_error() {
        let (reg, ctx) = make_ctx();
        let expr = Expr::FunctionCall {
            name: "NONEXISTENT".to_string(),
            args: vec![],
            span: dummy_span(),
        };
        assert_eq!(run(&expr, &reg, ctx), Value::Error(ErrorKind::Name));
    }

    #[test]
    fn function_eager_with_registered_fn() {
        // Register a custom eager fn: DOUBLE(x) = x * 2
        let mut reg = Registry::new();
        reg.register_eager("DOUBLE", |args| {
            if let Some(Value::Number(n)) = args.first() {
                Value::Number(n * 2.0)
            } else {
                Value::Error(ErrorKind::Value)
            }
        });
        let ctx = Context::empty();
        let expr = Expr::FunctionCall {
            name: "DOUBLE".to_string(),
            args: vec![Expr::Number(21.0, dummy_span())],
            span: dummy_span(),
        };
        assert_eq!(run(&expr, &reg, ctx), Value::Number(42.0));
    }

    #[test]
    fn function_eager_propagates_arg_error() {
        let mut reg = Registry::new();
        reg.register_eager("IDENTITY", |args| args.first().cloned().unwrap_or(Value::Empty));
        let mut vars = std::collections::HashMap::new();
        vars.insert("E".to_string(), Value::Error(ErrorKind::Ref));
        let ctx = Context::new(vars);
        let expr = Expr::FunctionCall {
            name: "IDENTITY".to_string(),
            args: vec![Expr::Variable("E".to_string(), dummy_span())],
            span: dummy_span(),
        };
        assert_eq!(run(&expr, &reg, ctx), Value::Error(ErrorKind::Ref));
    }

    // ── Error propagation in binary ops ──────────────────────────────────────

    #[test]
    fn error_propagates_left_in_add() {
        let (reg, ctx) = make_ctx();
        // "abc" + 1 => #VALUE! (left error from to_number)
        let expr = Expr::BinaryOp {
            op: BinaryOp::Add,
            left: Box::new(Expr::Text("abc".to_string(), dummy_span())),
            right: Box::new(Expr::Number(1.0, dummy_span())),
            span: dummy_span(),
        };
        assert_eq!(run(&expr, &reg, ctx), Value::Error(ErrorKind::Value));
    }

    #[test]
    fn error_propagates_left_over_right_in_cmp() {
        let (reg, _) = make_ctx();
        let mut vars = std::collections::HashMap::new();
        vars.insert("L".to_string(), Value::Error(ErrorKind::DivByZero));
        vars.insert("R".to_string(), Value::Error(ErrorKind::Ref));
        let ctx = Context::new(vars);
        let expr = Expr::BinaryOp {
            op: BinaryOp::Eq,
            left: Box::new(Expr::Variable("L".to_string(), dummy_span())),
            right: Box::new(Expr::Variable("R".to_string(), dummy_span())),
            span: dummy_span(),
        };
        assert_eq!(run(&expr, &reg, ctx), Value::Error(ErrorKind::DivByZero));
    }

    // ── Non-finite arithmetic ────────────────────────────────────────────────

    #[test]
    fn pow_negative_fractional_exponent_is_num_error() {
        // (-8.0).powf(0.333) → NaN → #NUM!
        let (reg, ctx) = make_ctx();
        let expr = Expr::BinaryOp {
            op: BinaryOp::Pow,
            left: Box::new(Expr::Number(-8.0, dummy_span())),
            right: Box::new(Expr::Number(0.333, dummy_span())),
            span: dummy_span(),
        };
        assert_eq!(run(&expr, &reg, ctx), Value::Error(ErrorKind::Num));
    }

    #[test]
    fn pow_overflow_to_infinity_is_num_error() {
        // 1e308.powf(2.0) → infinity → #NUM!
        let (reg, ctx) = make_ctx();
        let expr = Expr::BinaryOp {
            op: BinaryOp::Pow,
            left: Box::new(Expr::Number(1e308, dummy_span())),
            right: Box::new(Expr::Number(2.0, dummy_span())),
            span: dummy_span(),
        };
        assert_eq!(run(&expr, &reg, ctx), Value::Error(ErrorKind::Num));
    }

    // ── Lazy function dispatch ───────────────────────────────────────────────

    #[test]
    fn function_lazy_receives_raw_args() {
        // Register a lazy fn that reads a variable from ctx — proves EvalCtx is passed correctly
        // and args are NOT pre-evaluated.
        let mut reg = Registry::new();
        reg.register_lazy("LAZY_VAR", |args, ctx| {
            // The first arg is a Variable expr; evaluate it manually to prove ctx is live
            evaluate_expr(&args[0], ctx)
        });
        let mut vars = std::collections::HashMap::new();
        vars.insert("X".to_string(), Value::Number(99.0));
        let ctx = Context::new(vars);
        let expr = Expr::FunctionCall {
            name: "LAZY_VAR".to_string(),
            args: vec![Expr::Variable("X".to_string(), dummy_span())],
            span: dummy_span(),
        };
        assert_eq!(run(&expr, &reg, ctx), Value::Number(99.0));
    }

    // ── Multi-arg eager error ordering ───────────────────────────────────────

    #[test]
    fn function_eager_first_error_wins_over_second() {
        let mut reg = Registry::new();
        reg.register_eager("ADD2", |args| {
            match (args.get(0), args.get(1)) {
                (Some(Value::Number(a)), Some(Value::Number(b))) => Value::Number(a + b),
                _ => Value::Error(ErrorKind::Value),
            }
        });
        // arg[0] = #DIV/0!, arg[1] = #REF! → first error should win
        let mut vars = std::collections::HashMap::new();
        vars.insert("A".to_string(), Value::Error(ErrorKind::DivByZero));
        vars.insert("B".to_string(), Value::Error(ErrorKind::Ref));
        let ctx = Context::new(vars);
        let expr = Expr::FunctionCall {
            name: "ADD2".to_string(),
            args: vec![
                Expr::Variable("A".to_string(), dummy_span()),
                Expr::Variable("B".to_string(), dummy_span()),
            ],
            span: dummy_span(),
        };
        assert_eq!(run(&expr, &reg, ctx), Value::Error(ErrorKind::DivByZero));
    }

    // ── Concat error propagation ─────────────────────────────────────────────

    #[test]
    fn concat_right_error_propagates() {
        let (reg, _) = make_ctx();
        let mut vars = std::collections::HashMap::new();
        vars.insert("E".to_string(), Value::Error(ErrorKind::Value));
        let ctx = Context::new(vars);
        let expr = Expr::BinaryOp {
            op: BinaryOp::Concat,
            left: Box::new(Expr::Text("hello".to_string(), dummy_span())),
            right: Box::new(Expr::Variable("E".to_string(), dummy_span())),
            span: dummy_span(),
        };
        assert_eq!(run(&expr, &reg, ctx), Value::Error(ErrorKind::Value));
    }
}
