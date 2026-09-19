# ML Training API

Minimal Rust API scaffold using Actix Web with a contract-first starting point.

## Project shape

- `openapi/api.yaml` is the source of truth for the public API contract.
- `src/main.rs` implements the handlers that satisfy the current contract.
- `GET /openapi` serves the raw contract for local inspection.

## Run

```bash
cargo run
```

The API binds to `127.0.0.1:8080` by default. Override with `API_BIND_ADDRESS`.

## Verify

```bash
cargo test
curl http://127.0.0.1:8080/health
curl http://127.0.0.1:8080/v1/greetings/Ada
curl http://127.0.0.1:8080/openapi
```

## Next contract-first steps

1. Expand `openapi/api.yaml` before adding new handlers.
2. Keep response payloads and route parameters aligned with the contract.
3. Add integration tests for each contract change.