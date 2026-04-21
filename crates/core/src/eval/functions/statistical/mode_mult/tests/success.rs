use super::super::*;
use crate::types::{ErrorKind, Value};

#[test]
fn mode_mult_always_returns_ref() {
    // MODE.MULT is an array-spill function; scalar context always returns #REF!
    assert_eq!(
        mode_mult_fn(&[
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(2.0)
        ]),
        Value::Error(ErrorKind::Ref)
    );
}
