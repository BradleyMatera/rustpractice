# CrabCord Learning Checklist

## Done

- [x] Build and run a GPUI app shell.
- [x] Wire runtime asset loading from `assets/`.
- [x] Build a CrabCord-native theme pass (not generic clone styling).
- [x] Add local interactive state updates (`Send`, `Mic`, `Invite`).
- [x] Add right-panel mode switch (`Crew` <-> `Asset Desk`).
- [x] Add full PNG asset desk gallery with runtime rendering.
- [x] Replace skeleton server header with real in-product summary panel.
- [x] Add stable README screenshots for current crew and asset-desk views.
- [x] Simplify code to match current scope (single-screen architecture).
- [x] Split UI into focused files under `src/ui/shell/` to keep files smaller and easier to scan.
- [x] Update public docs (`README`, architecture, assets, release notes).
- [x] Add `.gitignore`, `LICENSE`, `CONTRIBUTING.md`, and `CHANGELOG.md`.

## Next

- [ ] Add message list virtualization once conversation depth grows.
- [ ] Add keyboard interactions and focus states for all clickable controls.
- [ ] Add persistence for local settings (mode, mic state, last channel).
- [ ] Add tests when there is enough logic to justify them.
- [ ] Add CI checks (formatting, clippy, build) on pull requests.
