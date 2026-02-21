# CrabCord

CrabCord is a Rust + GPUI desktop shell prototype inspired by modern chat clients.

It is currently a single-screen app focused on:
- fast startup and simple local state updates
- clean runtime asset loading
- minimal architecture until complexity justifies expansion

## Status

This repository is an early-stage UI prototype, not a production chat client yet.

## Requirements

- Rust (stable), installed via `rustup`
- macOS 13+ (current tested target)
- Git (optional, for cloning)

## Quick Start

1. Clone the repository:
```bash
git clone https://github.com/<owner>/<repo>.git
cd gpui-playground
```

2. Build and run:
```bash
cargo run
```

3. Optional release build:
```bash
cargo run --release
```

## How To Use The Current Build

When the app opens:
- click `Cycle Status` to rotate through status messages and increment the click counter
- click `Try Stream Action` to update the status line to the current stream placeholder state

## Project Layout

```text
gpui-playground/
  assets/
    brand/
    mock/
  docs/
    ARCHITECTURE.md
    ASSETS.md
    CHECKLIST.md
    RELEASE.md
  src/
    assets.rs
    main.rs
    ui/
      mod.rs
      shell.rs
```

## Packaging Notes

Release and macOS app-bundle notes are documented in `docs/RELEASE.md`.

## Publish To GitHub

If this directory is not already a Git repository:

```bash
git init
git add .
git commit -m "Initial public commit"
git branch -M main
git remote add origin https://github.com/<owner>/<repo>.git
git push -u origin main
```

## Screenshot

![CrabCord shell placeholder](assets/mock/crabcord-shell-1280x800.svg)

## License

This project is licensed under the MIT License. See `LICENSE`.

## Contributing

See `CONTRIBUTING.md` for local checks and pull request expectations.

## Changelog

See `CHANGELOG.md`.
