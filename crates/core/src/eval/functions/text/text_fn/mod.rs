use crate::display::display_number;
use crate::eval::coercion::{to_number, to_string_val};
use crate::eval::functions::check_arity;
use crate::types::Value;

fn apply_format(n: f64, fmt: &str) -> String {
    if let Some(dot_pos) = fmt.find('.') {
        let decimal_part = &fmt[dot_pos + 1..];
        if decimal_part.chars().all(|c| c == '0' || c == '#') {
            let places = decimal_part.len();
            return format!("{:.prec$}", n, prec = places);
        }
    } else if fmt.chars().all(|c| c == '0' || c == '#') {
        return format!("{:.0}", n);
    }
    display_number(n)
}

/// `TEXT(value, format_text)` — converts a number to a formatted string.
/// Supports `"0"`, `"0.0"`, `"0.00"` etc. (and `#` variants).
/// Falls back to `display_number` for unsupported formats.
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
    Value::Text(apply_format(n, &format))
}

#[cfg(test)]
mod tests;
