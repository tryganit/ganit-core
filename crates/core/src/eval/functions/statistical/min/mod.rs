use crate::types::Value;

/// `MIN(value1, ...)` — smallest numeric value in the arguments.
/// Ignores Text, Bool, Empty, Error. Returns `Value::Number(0.0)` if no numeric values are present.
pub fn min_fn(args: &[Value]) -> Value {
    let mut result: Option<f64> = None;
    for arg in args {
        if let Value::Number(n) = arg {
            result = Some(match result {
                None => *n,
                Some(cur) => cur.min(*n),
            });
        }
    }
    Value::Number(result.unwrap_or(0.0))
}

#[cfg(test)]
mod tests;
