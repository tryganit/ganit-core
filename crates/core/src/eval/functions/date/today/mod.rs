use crate::types::{ErrorKind, Value};

pub fn today_fn(args: &[Value]) -> Value {
    let _ = args;
    Value::Error(ErrorKind::Name)
}

#[cfg(test)]
mod tests;
