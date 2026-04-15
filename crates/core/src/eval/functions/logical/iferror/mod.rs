use crate::eval::functions::check_arity;
use crate::types::{ErrorKind, Value};

/// `IFERROR(value, error_val)` — returns value unless it is any error, then error_val.
pub fn iferror_fn(args: &[Value]) -> Value {
    if let Some(err) = check_arity(args, 2, 2) {
        return err;
    }
    match &args[0] {
        Value::Error(_) => args[1].clone(),
        _ => args[0].clone(),
    }
}

/// `IFNA(value, na_val)` — returns value unless it is `#N/A`, then na_val.
pub fn ifna_fn(args: &[Value]) -> Value {
    if let Some(err) = check_arity(args, 2, 2) {
        return err;
    }
    match &args[0] {
        Value::Error(ErrorKind::NA) => args[1].clone(),
        _ => args[0].clone(),
    }
}

#[cfg(test)]
mod tests;
