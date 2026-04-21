use crate::types::{ErrorKind, Value};

/// `MODE.MULT(value1, ...)` — array-spill function; returns `#REF!` in scalar context.
/// Google Sheets returns `#REF!` when MODE.MULT is not entered as an array formula.
pub fn mode_mult_fn(_args: &[Value]) -> Value {
    Value::Error(ErrorKind::Ref)
}

#[cfg(test)]
mod tests;
