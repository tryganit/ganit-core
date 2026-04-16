use crate::eval::coercion::to_string_val;
use crate::eval::functions::check_arity;
use crate::types::Value;

/// `SPLIT(text, delimiter)` — splits text by delimiter, returns an array.
pub fn split_fn(args: &[Value]) -> Value {
    if let Some(err) = check_arity(args, 2, 2) {
        return err;
    }
    let text = match to_string_val(args[0].clone()) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let delimiter = match to_string_val(args[1].clone()) {
        Ok(s) => s,
        Err(e) => return e,
    };
    let parts: Vec<Value> = text
        .split(delimiter.as_str())
        .map(|s| Value::Text(s.to_string()))
        .collect();
    Value::Array(parts)
}

#[cfg(test)]
mod tests;
