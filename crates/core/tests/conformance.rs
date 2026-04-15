//! Oracle conformance tests against Google Sheets reference values.
//!
//! Each `.xlsx` file under `tests/fixtures/m1/` is a Google Sheets export.
//! Each tab = one function name.  Each data row (skipping the header) has:
//!
//!   col A  description string
//!   col B  formula text stored as a plain string (e.g. "=ABS(-5)")
//!   col C  oracle value — the result Google Sheets produced
//!   col D  test category (basic / edge / coercion / error / nested)
//!
//! The test evaluates the formula in col B with `ganit_core::evaluate`, then
//! compares against the oracle in col C.  Number comparisons allow a small
//! floating-point tolerance.  Rows whose oracle cell is empty are skipped.

use calamine::{open_workbook, CellErrorType, Data, Reader, Xlsx};
use ganit_core::{evaluate, ErrorKind, Value};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// helpers
// ---------------------------------------------------------------------------

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/m1")
        .join(name)
}

/// Parse an error string like "#DIV/0!" into an ErrorKind, or return None.
fn parse_error_string(s: &str) -> Option<ErrorKind> {
    match s {
        "#DIV/0!" => Some(ErrorKind::DivByZero),
        "#VALUE!" => Some(ErrorKind::Value),
        "#REF!"   => Some(ErrorKind::Ref),
        "#NAME?"  => Some(ErrorKind::Name),
        "#NUM!"   => Some(ErrorKind::Num),
        "#N/A"    => Some(ErrorKind::NA),
        "#NULL!"  => Some(ErrorKind::Null),
        _         => None,
    }
}

fn oracle_to_value(cell: &Data) -> Option<Value> {
    match cell {
        Data::Float(f)  => Some(Value::Number(*f)),
        Data::Int(i)    => Some(Value::Number(*i as f64)),
        // Google Sheets exports error cells as plain strings in xlsx; check both.
        Data::String(s) => {
            if let Some(kind) = parse_error_string(s.trim()) {
                Some(Value::Error(kind))
            } else {
                Some(Value::Text(s.clone()))
            }
        }
        Data::Bool(b)   => Some(Value::Bool(*b)),
        Data::Error(e)  => Some(Value::Error(match e {
            CellErrorType::Div0  => ErrorKind::DivByZero,
            CellErrorType::Value => ErrorKind::Value,
            CellErrorType::Ref   => ErrorKind::Ref,
            CellErrorType::Name  => ErrorKind::Name,
            CellErrorType::Num   => ErrorKind::Num,
            CellErrorType::NA    => ErrorKind::NA,
            CellErrorType::Null  => ErrorKind::Null,
            _                    => return None,
        })),
        Data::Empty | Data::DateTimeIso(_) | Data::DurationIso(_) | Data::DateTime(_) => None,
    }
}

fn values_match(actual: &Value, expected: &Value) -> bool {
    match (actual, expected) {
        (Value::Number(a), Value::Number(b)) => {
            // relative tolerance 1e-9 with absolute floor 1e-10
            (a - b).abs() <= b.abs() * 1e-9 + 1e-10
        }
        _ => actual == expected,
    }
}

fn run_fixture(path: &Path) {
    assert!(path.exists(), "fixture not found: {:?}", path);

    let mut workbook: Xlsx<_> = open_workbook(path)
        .unwrap_or_else(|e| panic!("failed to open {:?}: {}", path, e));

    let sheet_names: Vec<String> = workbook.sheet_names().to_vec();
    let vars: HashMap<String, Value> = HashMap::new();
    let mut failures: Vec<String> = Vec::new();
    let mut total = 0usize;

    for sheet_name in &sheet_names {
        let range = workbook
            .worksheet_range(sheet_name)
            .unwrap_or_else(|e| panic!("failed to read sheet {sheet_name}: {e}"));

        for (row_idx, row) in range.rows().enumerate().skip(1) {
            if row.len() < 3 {
                continue;
            }

            let desc = match &row[0] {
                Data::String(s) => s.as_str(),
                _ => continue,
            };
            let formula = match &row[1] {
                Data::String(s) => s.as_str(),
                _ => continue,
            };
            let expected = match oracle_to_value(&row[2]) {
                Some(v) => v,
                None => continue,
            };

            total += 1;
            let actual = evaluate(formula, &vars);

            if !values_match(&actual, &expected) {
                failures.push(format!(
                    "  FAIL  [{sheet_name}] row {}  {desc}\n        formula:  {formula}\n        expected: {expected:?}\n        actual:   {actual:?}",
                    row_idx + 2,
                ));
            }
        }
    }

    if !failures.is_empty() {
        panic!(
            "\n{}/{} conformance failures in {}:\n\n{}\n",
            failures.len(),
            total,
            path.file_name().unwrap().to_string_lossy(),
            failures.join("\n\n"),
        );
    }
}

// ---------------------------------------------------------------------------
// one test per fixture file
// ---------------------------------------------------------------------------

macro_rules! conformance_test {
    ($fn_name:ident, $file:literal) => {
        #[test]
        fn $fn_name() {
            run_fixture(&fixture($file));
        }
    };
}

conformance_test!(m1_math_conformance,        "Math.xlsx");
conformance_test!(m1_logical_conformance,     "Logical.xlsx");
conformance_test!(m1_info_conformance,        "Info.xlsx");
conformance_test!(m1_statistical_conformance, "Statistical.xlsx");
conformance_test!(m1_operator_conformance,    "Operator.xlsx");
conformance_test!(m1_text_conformance,        "Text.xlsx");
