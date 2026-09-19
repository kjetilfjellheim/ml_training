# Project Guidelines

## Architecture
Keep this repository contract-first. Update `openapi/api.yaml` before changing public HTTP routes, request payloads, or response payloads.

## Build and Test
Use `cargo fmt`, `cargo clippy --all-targets --all-features`, and `cargo test` after Rust changes when practical.

## Conventions
Prefer small Actix handlers and shared route configuration functions over large `main` functions.
Keep JSON payload structs explicit and serializable with `serde` derives.
When the API surface changes, update the OpenAPI contract and matching tests in the same change.

### Naming
- Files and modules: `snake_case` (for example `api_contract.rs`, `service.rs`).
- Structs, enums, traits, and type aliases: `PascalCase` (for example `GreetingResponse`, `ApiService`).
- Functions and methods: `snake_case` (for example `configure_routes`, `openapi_contract`).
- Constants and statics: `SCREAMING_SNAKE_CASE` (for example `OPENAPI_SPEC`).
- Keep names descriptive and domain-driven; avoid abbreviations except widely accepted ones (for example `api`).