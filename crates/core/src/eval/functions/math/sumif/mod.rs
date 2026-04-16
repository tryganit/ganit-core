use crate::eval::functions::check_arity;
use crate::types::{ErrorKind, Value};
use super::criterion::{flatten_to_vec, matches_criterion, parse_criterion};

/// `SUMIF(range, criterion, [sum_range])` — sum the `sum_range` elements for which
/// the corresponding `range` element matches `criterion`.
///
/// If `sum_range` is omitted, `range` is both the test range and the sum range.
/// When `sum_range` is shorter than `range`, only positions that have a corresponding
/// `sum_range` value are considered (zip stops at the shorter sequence).
///
/// Non-finite results return `Value::Error(ErrorKind::Num)`.
pub fn sumif_fn(args: &[Value]) -> Value {
    if let Some(err) = check_arity(args, 2, 3) {
        return err;
    }
    let range = flatten_to_vec(&args[0]);
    let crit = parse_criterion(&args[1]);
    let sum_range: Vec<&Value> = if args.len() == 3 {
        flatten_to_vec(&args[2])
    } else {
        range.clone()
    };

    let mut total = 0.0_f64;
    for (r_val, s_val) in range.iter().zip(sum_range.iter()) {
        if matches_criterion(r_val, &crit) {
            match s_val {
                Value::Number(n) => total += n,
                Value::Bool(b) => total += if *b { 1.0 } else { 0.0 },
                Value::Text(s) => {
                    if let Ok(n) = s.parse::<f64>() {
                        total += n;
                    }
                    // Non-numeric text in sum_range: skip (Google Sheets behaviour)
                }
                _ => {} // Empty, Error: skip
            }
        }
    }
    if !total.is_finite() {
        return Value::Error(ErrorKind::Num);
    }
    Value::Number(total)
}

#[cfg(test)]
mod tests;
