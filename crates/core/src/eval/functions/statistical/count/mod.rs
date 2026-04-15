use crate::types::Value;

/// `COUNT(value1, ...)` — count of numeric `Value::Number` values only.
/// Ignores Text, Bool, Empty, Error. Returns 0 if called with no args.
pub fn count_fn(args: &[Value]) -> Value {
    let n = args.iter().filter(|v| matches!(v, Value::Number(_))).count();
    Value::Number(n as f64)
}

/// `COUNTA(value1, ...)` — count of non-empty values.
/// Counts Number, Text, Bool, Error — anything except `Value::Empty`.
pub fn counta_fn(args: &[Value]) -> Value {
    let n = args.iter().filter(|v| !matches!(v, Value::Empty)).count();
    Value::Number(n as f64)
}

#[cfg(test)]
mod tests;
