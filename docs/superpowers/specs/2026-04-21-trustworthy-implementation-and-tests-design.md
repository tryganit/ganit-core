# Trustworthy Implementation and Tests

**Date:** 2026-04-21
**Status:** Approved for implementation

---

## Problem Statement

truecalc v0.4.14 has 489 registered functions and ~70k tests, yet a systematic external audit found 20 bugs — 14 confirmed in code. Several are broken for all inputs (SUBTOTAL, SORTN), some for primary standalone use cases (SUMIFS/D* with inline arrays, BETA.INV 3-arg form, RANDARRAY multi-cell), and some are complete stubs that merged without any tests.

The root causes are structural:

1. **Self-confirming tests** — test authors test what they implemented, not what the spec requires. Stubs pass because no test ever calls them with a valid input and checks the output against an external oracle.
2. **No per-function conformance gate** — a function can be registered with zero fixture rows. 80% aggregate coverage hides zero-coverage functions.
3. **Fixture coverage is incomplete** — existing fixtures were written alongside implementations, not derived from documented signatures. Inline arrays, optional arg defaults, and extended match modes are systematically absent.
4. **Silent registry overwrites** — 9 function names are registered multiple times across modules. The last registration wins silently. Dead stub implementations in `filter/mod.rs` overwrite nothing but compile and confuse.
5. **Context-limited functions are misrepresented** — `INDIRECT`, `OFFSET`, `FORMULATEXT` etc. need a live cell grid. In the standalone evaluator they return `#N/A`, which looks like a bug rather than a design boundary.

---

## Goals

1. Every registered function has at least one passing conformance fixture row — stubs are structurally unshippable.
2. The fixture oracle is an external source of truth (Google Sheets, later Excel) — not the implementation itself.
3. The registry has a single authoritative registration per function name — duplicate registrations fail at startup.
4. Design boundaries are explicit — context-limited functions are removed from the standalone evaluator with a tracked issue for `@truecalc/sheets`.
5. The fixture generation process is reproducible and first-class — a `cargo xtask` command, not a one-time external script.

---

## Approach: Three Parallel Tracks

### Track 1 — Registry Integrity

Clean up the function registry so it is unambiguous and self-enforcing. All five sub-issues can start immediately and are independent of each other except T1.4 which requires T1.1.

#### T1.1 — Startup duplicate registration detection

Replace the silent `HashMap::insert` in `Registry::new()` with an assertion that panics if a function name is registered more than once. This fires on the first test run (effectively compile-time in CI).

```rust
fn register_eager(&mut self, name: &str, ...) {
    assert!(!self.map.contains_key(name),
        "duplicate function registration: '{}' — each name must be registered exactly once", name);
    ...
}
```

The 9 currently-duplicate names must be resolved before this lands:

| Function | Registered in | Resolution |
|----------|--------------|------------|
| SORT | filter (stub), array (real) | Remove filter stub |
| UNIQUE | operator (stub), filter (stub), array (real) | Remove operator and filter stubs |
| INDEX | filter (stub), array (real) | Remove filter stub |
| ROWS | filter (stub), array (real), lookup (lazy) | Resolve lazy/eager conflict — see T1.4 |
| COLUMNS | filter (stub), array (real), lookup (lazy) | Resolve lazy/eager conflict — see T1.4 |
| ERF | math (copy), engineering (copy) | Remove math copy |
| ERF.PRECISE | math (copy), engineering (copy) | Remove math copy |
| ERFC | math (copy), engineering (copy) | Remove math copy |
| ERFC.PRECISE | math (copy), engineering (copy) | Remove math copy |

#### T1.2 — Explicit alias registration

Functions with multiple names (e.g., `TTEST` / `T.TEST`, `BETAINV` / `BETA.INV`) must use a dedicated `register_alias` path rather than two independent registrations. This distinguishes intentional aliases from accidental duplicates.

```rust
fn register_alias(&mut self, alias: &str, canonical: &str) {
    // alias → same handler as canonical, documented in metadata as alias
}
```

#### T1.3 — Remove context-limited functions

The following functions require a live cell grid and cannot be correctly implemented in the standalone evaluator. Remove their registrations. Callers will receive `#NAME?` (function not found), which is honest.

- `INDIRECT`, `OFFSET`, `FORMULATEXT`, `GETPIVOTDATA`, `ISFORMULA`, `CELL`

Open a separate tracking issue: *"Implement grid-context functions in @truecalc/sheets package"*. Document the design boundary explicitly in the crate README: standalone evaluator = no grid, no cell references.

#### T1.4 — Remove filter module dead code

After T1.1 lands, the duplicate registration assertion makes these stubs unreachable. Remove them entirely from `filter/mod.rs`: SORT, UNIQUE, INDEX, ROWS, COLUMNS stub implementations.

Also resolve the ROWS/COLUMNS lazy/eager conflict: `lookup/mod.rs` registers them as lazy (expects `&[Expr]`), `array/mod.rs` as eager (expects `&[Value]`). Determine the correct dispatch path and consolidate to one registration.

#### T1.5 — Consolidate ERF/ERFC duplication

`math/mod.rs` and `engineering/mod.rs` contain identical implementations of ERF, ERF.PRECISE, ERFC, ERFC.PRECISE. Remove the copies in `math/mod.rs`. The engineering module is the canonical home.

---

### Track 2 — Regression Baseline

Establish an honest picture of current state and make CI enforce it. T2.1 is prerequisite for T2.2 and T2.3, which can then run in parallel.

#### T2.1 — Migrate fixtures from .xlsx to TSV

Replace the 30 `.xlsx` files in `tests/fixtures/m1-m4/` with TSV files under `tests/fixtures/google_sheets/`.

**New fixture format — 5 columns:**

| Col | Name | Content | Example |
|-----|------|---------|---------|
| A | `description` | Human-readable test name | `positive integer` |
| B | `formula_text` | Formula as text — apostrophe-prefixed in TSV to prevent spreadsheet evaluation | `=ABS(5)` |
| C | `expected_value` | Oracle value — filled by Sheets/Excel evaluation | `5` |
| D | `test_category` | `basic` / `edge` / `coercion` / `error` / `nested` | `basic` |
| E | `expected_type` | `number` / `string` / `boolean` / `error` / `array` | `number` |

**Col E semantics:**
- `number` — compare as `f64` with 1e-4 relative tolerance
- `string` — exact string equality
- `boolean` — compare as bool (`TRUE`/`FALSE`)
- `error` — compare error kind only (e.g., `#VALUE!` matches `#VALUE!` regardless of message)
- `array` — Col C contains `ARRAYTOTEXT` output; parse as array literal and compare element-wise

**Why TSV over CSV:** Excel uses system locale for CSV delimiters (semicolons in European locales). TSV is locale-independent and unambiguous across both oracles.

**Migration:** convert existing oracle values from `.xlsx` Col C to TSV Col C (values unchanged), infer `expected_type` from cell type in the xlsx (number cell → `number`, error cell → `error`, etc.), write to `tests/fixtures/google_sheets/{category}.tsv`. Delete `m1-m4/`.

Update the conformance runner to read from `tests/fixtures/google_sheets/`.

#### T2.2 — Add 14 known bug reproduction cases as failing fixture rows

Add one TSV row per confirmed bug to the appropriate category file. These rows will fail conformance immediately — that is the point. They become a red/green dashboard of bug-fix progress.

Table shorthand: `formula → value (type)` maps to Col B = formula, Col C = value, Col E = type in the actual TSV.

| Bug | Function | Formula | Expected value | Expected type |
|-----|----------|---------|----------------|---------------|
| BUG-01 | SUBTOTAL | `SUBTOTAL(9,{1,2,3})` | `6` | `number` |
| BUG-02 | SUMIFS | `SUMIFS({10,20,30},{1,2,3},">1")` | `50` | `number` |
| BUG-02 | AVERAGEIFS | `AVERAGEIFS({10,20,30},{1,2,3},">1")` | `25` | `number` |
| BUG-02 | MAXIFS | `MAXIFS({10,20,30},{1,2,3},">1")` | `30` | `number` |
| BUG-02 | MINIFS | `MINIFS({10,20,30},{1,2,3},">1")` | `20` | `number` |
| BUG-03 | T.TEST | `T.TEST({1,2,3},{4,5,6},2,1)` | `0.021` | `number` |
| BUG-04 | UNIQUE | `ARRAYTOTEXT(UNIQUE({1,2,2,3},FALSE,TRUE),1)` | `{3}` | `array` |
| BUG-05 | ISFORMULA | — | — | — |
| BUG-06 | RANDARRAY | — | — | — |
| BUG-08 | DSUM | `DSUM({"Name","Sales";"A",100;"B",200},"Sales",{"Sales",">100"})` | `200` | `number` |
| BUG-09 | SORT | `ARRAYTOTEXT(SORT({3,1,2},1,-1),1)` | `{3,2,1}` | `array` |
| BUG-10 | SORTBY | `ARRAYTOTEXT(SORTBY({3,1,2},{2,1,3},-1),1)` | `{3,2,1}` | `array` |
| BUG-11 | SORTN | `ARRAYTOTEXT(SORTN({5,3,1,4,2},3),1)` | `{1,2,3}` | `array` |
| BUG-12 | XLOOKUP | `XLOOKUP("b*",{"a","ba","bc"},{1,2,3},,2)` | `2` | `number` |
| BUG-13 | XMATCH | `XMATCH("b*",{"a","ba","bc"},2)` | `2` | `number` |
| BUG-14 | MATCH | `MATCH("b*",{"a","ba","bc"},0)` | `2` | `number` |
| BUG-16 | TEXT | `TEXT(0.5,"hh:mm:ss")` | `12:00:00` | `string` |
| BUG-17 | VALUE | `VALUE("1,234.56")` | `1234.56` | `number` |
| BUG-19 | BETA.INV | `BETA.INV(0.5,2,2)` | `0.5` | `number` |
| BUG-20 | COLUMN | `ARRAYTOTEXT(COLUMN({1,2,3}),1)` | `{1,2,3}` | `array` |

**BUG-05 (ISFORMULA):** addressed by T1.3 (function removed from standalone evaluator — no fixture row needed).
**BUG-06 (RANDARRAY):** volatile function — addressed by T3.11 (property tests), not conformance fixtures.
**BUG-08 remaining D* functions:** DSUM row above covers the pattern; the remaining 11 D* functions (DAVERAGE, DCOUNT, DCOUNTA, DGET, DMAX, DMIN, DPRODUCT, DSTDEV, DSTDEVP, DVAR, DVARP) each get one equivalent row.

#### T2.3 — CI gate: every registered function needs ≥1 passing fixture row

Add a CI check that fails if any registered function has zero passing conformance rows, with two explicit exclusions:

- **Volatile functions** (`RAND`, `RANDARRAY`, `NOW`, `TODAY`, `RANDBETWEEN`) — excluded from conformance, must have property tests instead
- **Context-limited functions** — removed in T1.3, no longer registered

Implementation: after the conformance suite runs, cross-reference the function registry against passing fixture rows. Any registered function with zero passing rows fails CI with a message naming the function.

This makes stubs structurally unshippable: a new function cannot merge until it has at least one passing fixture row.

---

### Track 3 — Generator and Systematic Coverage

Build the fixture generator as a first-class tool, generate systematic coverage across all 489 functions, evaluate against Google Sheets oracle, land fixtures. T3.1 is prerequisite for T3.2–T3.7, which are fully parallel. T3.8 (manual oracle step) requires T3.2–T3.7 to be complete.

#### T3.1 — Scaffold `cargo xtask generate-fixtures`

Build the CLI entry point and shared generator infrastructure.

```bash
cargo xtask generate-fixtures --platform sheets --category math
cargo xtask generate-fixtures --platform sheets --all
cargo xtask generate-fixtures --platform excel --category math
```

**Output:** `target/fixture-gen/{platform}/{category}.tsv` (gitignored)

**Shared infrastructure:**
- `TestCase` struct: description, formula string, test_category, expected_type
- `GeneratorConfig`: platform (sheets/excel), output directory
- `wrap_array(formula, platform)` — platform-specific array output wrapping:
  - Sheets: `=ARRAYTOTEXT({formula}, 1)`
  - Excel: `=TEXTJOIN(",", TRUE, {formula})`
- `emit_tsv(cases, path)` — writes TSV with Col B apostrophe-prefixed, Col C as live formula, Col E as expected_type
- Volatile function registry — functions excluded from generation
- Context-limited function registry — functions excluded from generation

**Coverage definition per function ("systematic D"):**

For every registered non-volatile non-context-limited function, the generator produces test cases covering:
1. Every valid argument count: required-only, each optional argument added one at a time, maximum
2. Every input shape per argument: scalar, 1D row array, 1D column array, 2D array
3. Every value type per argument: number, text, boolean, empty/blank
4. Common error cases: wrong argument type, out-of-range value, division by zero where applicable
5. Coercion cases: text-that-looks-like-number, boolean-as-number

For lookup functions: lookup value present ≥50% of rows, absent ≥25%, wildcard-matchable ≥25%.
For variadic functions (CHOOSE, IFS, CONCAT): min args, min+1, min+2, max-2, max.

#### T3.2–T3.7 — Generator implementation per category

Each sub-issue implements the generator for one category group. All can run in parallel after T3.1.

| Issue | Category | Functions | Notes |
|-------|----------|-----------|-------|
| T3.2 | math + operator | 78 + 1 | Includes SUBTOTAL fix in generator |
| T3.3 | statistical | 137 | Large; paired/unpaired distributions need valid input ranges |
| T3.4 | array + filter | 33 + 2 | All array-output functions use `wrap_array()` |
| T3.5 | lookup | 17 | Semantically-aware inputs — lookup value in range |
| T3.6 | text + date + engineering + financial + parser + web | 41+26+52+50+6+3 | Largest batch; independent by sub-category |
| T3.7 | database | 12 | 2D array inputs for all 12 D* functions |

#### T3.8 — Build Sheets oracle adapter

A script that takes the generator output TSVs and produces oracle-evaluated fixture TSVs via the Google Sheets API (not manual export — the API returns typed JSON, which the adapter uses to populate Col E accurately).

```
target/fixture-gen/google_sheets/{category}.tsv   (generator output — formulas)
        │
        Sheets API: evaluate each Col C formula, get typed response
        │
        ▼
tests/fixtures/google_sheets/{category}.tsv        (oracle values in Col C, types confirmed in Col E)
```

The adapter maps Sheets API response types:
- `NUMBER` → `number`, raw float in Col C
- `STRING` → `string`, raw string in Col C
- `BOOL` → `boolean`, `TRUE`/`FALSE` in Col C
- `ERROR` → `error`, error code string in Col C (`#VALUE!`, `#N/A`, etc.)

For `expected_type: array` rows, the adapter captures the ARRAYTOTEXT result and stores it in Col C.

#### T3.9 — Run generator + oracle adapter, review output, land fixture TSVs

This is a human-triggered step (not automated CI), performed once after T3.2–T3.8 are complete. The evaluation itself is automated via the Sheets API adapter built in T3.8.

```bash
# 1. Generate all intermediate TSVs
cargo xtask generate-fixtures --platform sheets --all
# → writes to target/fixture-gen/google_sheets/{category}.tsv

# 2. Run oracle adapter — calls Sheets API, populates Col C and confirms Col E
cargo xtask oracle-evaluate --platform sheets --all
# → writes to tests/fixtures/google_sheets/{category}.tsv
# (requires GOOGLE_SHEETS_API_KEY env var)

# 3. Review diff: new fixture rows, any unexpected oracle values
# 4. Check into branch — CI will show red rows for each failing function
```

The failing rows in CI drive T3.10.

#### T3.10 — Fix all failing fixture rows

Fix every function that fails conformance after T3.9 lands. This includes the 14 known bugs and any newly surfaced failures. Each bug fix is a self-contained change: the failing row is the test, the fix makes it green.

Priority order (from bug report):
1. BUG-01 SUBTOTAL, BUG-08 D* functions, BUG-11 SORTN, BUG-06 RANDARRAY — entire families non-functional
2. BUG-04 UNIQUE exactly_once, BUG-09/10 SORT/SORTBY descending, BUG-19 BETA.INV defaults — likely small fixes
3. BUG-02 \*IFS with arrays, BUG-12/13/14 wildcard matching — align array-literal handling
4. BUG-16 TEXT time formats, BUG-17 VALUE formatted strings, BUG-20 COLUMN/ROW arrays

#### T3.11 — Property tests for volatile functions

Volatile functions cannot have Sheets oracle fixtures (output changes on every evaluation). They must have property tests instead.

| Function | Properties to test |
|----------|-------------------|
| `RAND()` | result in [0, 1); successive calls differ |
| `RANDARRAY(r,c)` | shape is r×c; all cells differ from each other; all in [0, 1) |
| `RANDBETWEEN(lo,hi)` | result is integer in [lo, hi] |
| `NOW()` | result > known past timestamp; result < known future timestamp |
| `TODAY()` | same as NOW() but integer (no fractional day) |

The RANDARRAY cell-independence bug (BUG-06) is fixed when this test lands and passes.

---

## Fixture Directory Structure

```
crates/core/tests/
  fixtures/
    google_sheets/              ← committed, oracle-verified (replaces m1-m4/)
      math.tsv
      statistical.tsv
      array.tsv
      filter.tsv
      lookup.tsv
      text.tsv
      date.tsv
      engineering.tsv
      financial.tsv
      logical.tsv
      database.tsv
      parser.tsv
      web.tsv
      operator.tsv
    excel/                      ← added when Excel work begins (T3.12, separate epic)

target/
  fixture-gen/                  ← gitignored, generator output pre-oracle
    google_sheets/
      math.tsv
      ...
    excel/
      ...
```

---

## What Is Not Addressed (Documented Boundaries)

**Volatile functions** (`RAND`, `RANDARRAY`, `NOW`, `TODAY`, `RANDBETWEEN`): no stable oracle value exists. Covered by property tests only (T3.11). Excluded from the CI per-function gate.

**Context-limited functions** (`INDIRECT`, `OFFSET`, `FORMULATEXT`, `GETPIVOTDATA`, `ISFORMULA`, `CELL`): require a live cell grid. Removed from the standalone evaluator in T1.3. Tracked in a separate issue for `@truecalc/sheets`.

**Excel oracle** (T3.12): out of scope for this epic. The directory structure (`tests/fixtures/excel/`) and the oracle-agnostic generator architecture accommodate it. Tracked in a separate issue.

**Known deviations**: the `KNOWN_DEVIATIONS` list in `conformance_reporter.rs` stays empty. Where truecalc currently differs from Google Sheets behaviour (e.g. `MID` with `start=0` returning `#NUM!` instead of clamping to 1), the correct action is to fix the implementation to match the spec, not to document the divergence. Deviations are only recorded when there is a deliberate, permanent architectural reason to differ.

---

## Epic Structure — 20 Sub-Issues

### Track 1: Registry Integrity
- **T1.1** Add startup duplicate registration detection
- **T1.2** Add explicit alias registration path
- **T1.3** Remove context-limited functions; open @truecalc/sheets tracking issue
- **T1.4** Remove filter module dead code and resolve ROWS/COLUMNS lazy/eager conflict *(depends on T1.1)*
- **T1.5** Consolidate ERF/ERFC duplication

### Track 2: Regression Baseline
- **T2.1** Migrate fixtures from .xlsx to TSV with 5-column format
- **T2.2** Add 14 known bug reproduction cases as failing fixture rows *(depends on T2.1)*
- **T2.3** Add CI gate — every registered non-volatile function needs ≥1 passing fixture row *(depends on T2.1)*

### Track 3: Generator and Systematic Coverage
- **T3.1** Scaffold `cargo xtask generate-fixtures` CLI and shared infrastructure *(depends on T2.1)*
- **T3.2** Generator: math + operator *(depends on T3.1)*
- **T3.3** Generator: statistical *(depends on T3.1)*
- **T3.4** Generator: array + filter *(depends on T3.1)*
- **T3.5** Generator: lookup *(depends on T3.1)*
- **T3.6** Generator: text + date + engineering + financial + parser + web *(depends on T3.1)*
- **T3.7** Generator: database *(depends on T3.1)*
- **T3.8** Build Sheets oracle adapter *(depends on T3.1)*
- **T3.9** [Manual] Run generator → oracle → land fixture TSVs *(depends on T3.2–T3.8)*
- **T3.10** Fix all failing fixture rows *(depends on T3.9)*
- **T3.11** Add property tests for volatile functions *(depends on T3.1)*

**T3.12** Build Excel oracle adapter — out of scope, separate epic

---

## Success Criteria

- Zero registered non-volatile functions with zero passing conformance rows
- All 14 confirmed bugs have passing fixture rows
- `cargo xtask generate-fixtures --platform sheets --all` produces valid TSV for all 489 functions
- `cargo xtask oracle-evaluate --platform sheets --all` produces populated fixture TSVs
- Duplicate registration panics at test startup — confirmed by a test that attempts double-registration
- `tests/fixtures/google_sheets/` contains TSV files; `m1-m4/` is deleted
- CI is green on main
