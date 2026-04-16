use crate::types::{ErrorKind, Value};

pub fn workday_intl_fn(args: &[Value]) -> Value {
    let _ = args;
    Value::Error(ErrorKind::Name)
}

#[cfg(test)]
mod tests;
