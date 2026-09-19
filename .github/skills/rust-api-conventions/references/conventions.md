# Rust API Conventions Reference

## Workflow

1. Edit `openapi/api.yaml` first.
2. Implement or update the Actix handler.
3. Verify the route is registered in the shared configuration function.
4. Add or update tests for the changed behavior.

## Verification

```bash
cargo fmt
cargo clippy --all-targets --all-features
cargo test
```

## Naming Conventions

- Files and modules: `snake_case`.
- Structs, enums, traits, and type aliases: `PascalCase`.
- Functions and methods: `snake_case`.
- Constants and statics: `SCREAMING_SNAKE_CASE`.
- Prefer explicit, domain-oriented names over shortened forms.

## Current Endpoints

- `GET /health`
- `GET /v1/greetings/{name}`
- `GET /openapi`