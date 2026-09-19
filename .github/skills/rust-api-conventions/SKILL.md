---
name: rust-api-conventions
description: 'Apply the repository conventions for Actix Web API work. Use when editing Rust handlers, updating OpenAPI contracts, adding endpoints, or keeping contract-first tests aligned.'
user-invocable: true
---

# Rust API Conventions

Use this skill when working on the API surface of this repository.

## What This Skill Covers

- Contract-first changes for HTTP endpoints
- Actix Web handler structure
- Naming conventions for Rust API code
- Test expectations for route and payload changes

## Procedure

1. Read `openapi/api.yaml` first and treat it as the contract source of truth.
2. Update the contract before changing route paths, parameters, or response shapes.
3. Keep Actix handlers small and wire them through a shared route configuration function.
4. Follow naming conventions from [conventions](./references/conventions.md) for files, structs, and functions.
5. Add or update tests for every contract change.
6. Run the verification commands described in [conventions](./references/conventions.md).