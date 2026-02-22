# Contributing

## Local Setup

```bash
cargo check
cargo check --features axiom-backend
cargo run
```

## Before Opening A Pull Request

Run these commands from the repository root:

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo check
cargo check --features axiom-backend
```

## Pull Request Expectations

- Keep changes scoped to a single objective.
- Update docs when behavior or commands change.
- If UI layout/behavior changes, update `docs/screenshots/` and `assets/mock/crabcord-shell-1280x800.png`.
- Avoid introducing new architectural layers until there is demonstrated reuse or complexity pressure.
