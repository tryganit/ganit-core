# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/tryganit/ganit/releases/tag/ganit-core-v0.1.0) - 2026-04-15

### Added

- *(core)* add property-based tests for all function categories — issue #12
- *(core)* implement 5 statistical functions for issue #7
- *(core)* implement financial functions (PMT, NPV, IRR, PV, FV, RATE, NPER) — issue #6
- *(core)* implement 14 text functions for issue #5
- *(core)* add evaluate() public API and integration tests
- *(core)* implement SUM (eager) and IF (lazy) with test directory structure
- *(core)* implement evaluate_expr AST walker
- *(core)* add Eager/Lazy function registry with EvalCtx
- *(core)* add evaluator context and Excel type coercion
- *(core)* implement nom recursive-descent parser with full operator precedence
- *(core)* add parser token helpers
- *(core)* add AST types (Expr, Span, operators)
- *(core)* add Value, ErrorKind, ParseError types and display_number

### Fixed

- *(core)* assert result variant in proptest blocks, fix trim strategy
- *(core)* remove unreachable error arms from max/min/median
- *(core)* correct PMT type=1 formula and RATE r=0 derivative
- *(core)* TEXT respects decimal-place format strings
- *(core)* guard sum and number literals against non-finite values
- *(core)* normalize Context keys to uppercase for case-insensitive lookup
- *(core)* normalize -0.0 in display_number, document Value::Number NaN invariant

### Other

- Merge pull request #23 from tryganit/issue/4-logical-functions
- Merge pull request #21 from tryganit/issue/5-text-functions
- *(core)* add developer doc comments and restructure tests into success/failure/edge dirs
- *(core)* clean up public API surface and document evaluate() cost
- *(core)* unify lazy arity check and improve test coverage
- *(core)* fix test names and add over-arity tests
- *(core)* guard non-finite results and add missing evaluator tests
- *(core)* add NaN/infinity assertions to display_number tests
- Set up Cargo workspace with core, wasm, and mcp crates
