# CrabCord

CrabCord is a Rust + GPUI desktop shell with a custom CrabCord theme, PNG asset pipeline, and an Axiom-ready backend command boundary.

Current scope:
- single-window multi-lane chat layout
- guild rail, channel/DM list, timeline, crew panel, and asset desk
- interactive controls wired to state and backend commands
- fake workspace data (guilds/channels/DMs/messages/members) from `src/backend.rs`
- PNG-only runtime asset library
- split UI modules under `src/ui/shell/*.rs`
- no persistence yet

## Screenshots

Crew view:

![CrabCord crew view](docs/screenshots/crabcord-crew-view.png)

Asset Desk view:

![CrabCord asset desk view](docs/screenshots/crabcord-asset-desk-view.png)

## Requirements

- macOS (currently tested on macOS only)
- Rust stable (`rustup`)
- Xcode + Xcode Command Line Tools
- internet access on first build (downloads crates and GPUI git dependency)

Install missing macOS tooling:

```bash
xcode-select --install
```

Quick verification:

```bash
rustc --version
xcode-select -p
xcrun --find metal
```

## Quick Start

```bash
git clone <repo-url>
cd gpui-playground
cargo run
```

Run with Axiom command conversion enabled:

```bash
cargo run --features axiom-backend
```

Release run:

```bash
cargo run --release
```

## Current Interaction Model

- Click guilds in the left rail to switch server context
- Click channels / DMs to switch active conversation
- `Send`: appends a new fake message in the active conversation
- `Mic`: toggles local mic state (`Live`/`Muted`)
- `Invite`: increments crew count in Crew mode
- `Open Asset Desk` / `Back to Crew`: toggles right panel mode

## Project Layout

```text
gpui-playground/
  assets/
    brand/
    mock/
    ui/
  docs/
    ARCHITECTURE.md
    ASSETS.md
    CHECKLIST.md
    RELEASE.md
    screenshots/
  src/
    assets.rs
    backend.rs
    main.rs
    ui/
      elements.rs
      theme.rs
      shell.rs
      shell/
        render.rs
        left_panel.rs
        chat_panel.rs
        right_panel.rs
```

## Notes

- UI state and interaction handlers live in `src/ui/shell.rs`.
- Fake data models and backend command types live in `src/backend.rs`.
- Runtime assets load from `assets/` via `FileAssetSource`.
- `main.rs` creates a backend channel worker and injects `ChannelBackend` into the UI shell.
- Packaging/release flow is documented in `docs/RELEASE.md`.

## Troubleshooting

If you hit `xcrun: error: unable to find utility "metal"`:

```bash
sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer
```

If you hit `'dispatch/dispatch.h' file not found`:

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
- Zed macOS setup notes: https://zed.dev/docs/development/macos

## License

MIT. See `LICENSE`.
