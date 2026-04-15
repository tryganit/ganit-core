# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/tryganit/ganit/releases/tag/ganit-wasm-v0.1.0) - 2026-04-15

### Added

- *(wasm)* implement evaluate, validate, list_functions WASM bindings — issue #8

### Fixed

- *(wasm)* enable nontrapping-float-to-int in wasm-opt and return #NUM! for out-of-range JSON numbers
- *(wasm)* Array type string, document variables param, remove unused dep

### Other

- Merge pull request #28 from tryganit/issue/13-release
- Fix wasm-opt: pass --enable-bulk-memory flag
- Set up Cargo workspace with core, wasm, and mcp crates
