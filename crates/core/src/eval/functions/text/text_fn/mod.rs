use crate::display::display_number;
use crate::eval::coercion::{to_number, to_string_val};
use crate::eval::functions::check_arity;
use crate::types::Value;

/// `TEXT(value, format_text)` — converts a number to a formatted string.
/// For format "0" returns integer string; otherwise uses display_number.
pub fn text_fn(args: &[Value]) -> Value {
    if let Some(err) = check_arity(args, 2, 2) {
        return err;
    }
    let n = match to_number(args[0].clone()) {
        Ok(n) => n,
        Err(e) => return e,
    };
    let format = match to_string_val(args[1].clone()) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let result = if format == "0" {
        format!("{}", n as i64)
    } else {
        display_number(n)
    };
    Value::Text(result)
}

#[cfg(test)]
mod tests;
