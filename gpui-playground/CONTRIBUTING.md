# Contributing

## Local Setup

```bash
cargo check
cargo run
```

## Before Opening A Pull Request

Run these commands from the repository root:

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo check
```

## Pull Request Expectations

- Keep changes scoped to a single objective.
- Update docs when behavior or commands change.
- Avoid introducing new architectural layers until there is demonstrated reuse or complexity pressure.
