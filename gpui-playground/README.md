# CrabCord

CrabCord is a Rust + GPUI desktop shell prototype inspired by modern chat clients.

It is currently a single-screen app focused on:
- fast startup and simple local state updates
- clean runtime asset loading
- minimal architecture until complexity justifies expansion

## Status

This repository is an early-stage UI prototype, not a production chat client yet.

## Requirements

- macOS (this project is currently tested on macOS only)
- Rust (stable), installed via `rustup`
- Xcode (full app install)
- Xcode Command Line Tools
- `cmake`
- Git (optional, for cloning)
- Internet access on first build (pulls crates and the `gpui` git dependency)

Run the prereq checker before building:

```bash
./scripts/check-prereqs.sh
```

Install missing macOS dependencies:

```bash
xcode-select --install
brew install cmake
```

Verify your machine is ready:

```bash
rustc --version
xcode-select -p
xcrun --find metal
cmake --version
```

## GPUI Dependency Notes

- This app depends on `gpui` and `gpui_platform` from the Zed repository.
- GPUI is currently pre-1.0 and may change quickly between upstream commits.
- This repository commits `Cargo.lock` so a normal `cargo run` uses the pinned dependency graph.
- First build still requires network access to fetch crates and git dependencies.

## Quick Start

1. Clone the repository:
```bash
git clone https://github.com/<owner>/<repo>.git
cd gpui-playground
```

2. Validate machine prerequisites:
```bash
./scripts/check-prereqs.sh
```

3. Build and run:
```bash
cargo run
```

4. Optional release build:
```bash
cargo run --release
```

## How To Use The Current Build

When the app opens:
- click `Cycle Status` to rotate through status messages and increment the click counter
- click `Try Stream Action` to update the status line to the current stream placeholder state

`cargo run` should work immediately after clone only when all requirements above are installed and configured.

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
  scripts/
    check-prereqs.sh
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

## Troubleshooting Build Setup

If you see `xcrun: error: unable to find utility "metal"`:

```bash
sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer
```

If you see `'dispatch/dispatch.h' file not found`:

```bash
xcode-select --install
sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer
sudo xcodebuild -license accept
export BINDGEN_EXTRA_CLANG_ARGS="--sysroot=$(xcrun --show-sdk-path)"
```

Then retry:

```bash
cargo clean
cargo run
```

## References

- GPUI docs: https://docs.rs/gpui/latest/gpui/
- Zed macOS dev requirements: https://zed.dev/docs/development/macos

## Screenshot

![CrabCord shell placeholder](assets/mock/crabcord-shell-1280x800.svg)

## License

This project is licensed under the MIT License. See `LICENSE`.

## Contributing

See `CONTRIBUTING.md` for local checks and pull request expectations.

## Changelog

See `CHANGELOG.md`.
