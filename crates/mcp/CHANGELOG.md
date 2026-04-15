# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0](https://github.com/tryganit/ganit/releases/tag/ganit-mcp-v0.1.0) - 2026-04-15

### Added

- *(mcp)* implement 5-tool MCP server over stdio — issue #9

### Fixed

- *(mcp)* JSON-RPC parse errors, isError flag, graceful EOF, remove hot-path unwrap

### Other

- Set up Cargo workspace with core, wasm, and mcp crates
