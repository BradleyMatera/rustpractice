# CrabCord Learning Checklist

## Done

- [x] Build and run a GPUI app shell.
- [x] Wire runtime asset loading from `assets/`.
- [x] Build a CrabCord-native theme pass (not generic clone styling).
- [x] Add interactive state updates (`Send`, `Mic`, `Invite`, lane selection).
- [x] Add right-panel mode switch (`Crew` <-> `Asset Desk`).
- [x] Add full PNG asset desk gallery with runtime rendering.
- [x] Replace placeholder shell with a full multi-lane app layout.
- [x] Add stable README screenshots for current crew and asset-desk views.
- [x] Simplify code to match current scope (single-screen architecture).
- [x] Split UI into focused files under `src/ui/shell/` to keep files smaller and easier to scan.
- [x] Add backend command boundary and fake workspace seed data in `src/backend.rs`.
- [x] Wire optional Axiom feature compilation path.
- [x] Update public docs (`README`, architecture, assets, release notes).
- [x] Add `.gitignore`, `LICENSE`, `CONTRIBUTING.md`, and `CHANGELOG.md`.

## Next

- [ ] Add typed message input state (replace quick canned send text).
- [ ] Add keyboard interactions and focus states for all clickable controls.
- [ ] Add persistence for selected guild/conversation and mic state.
- [ ] Add an actor consumer that handles backend commands end-to-end.
- [ ] Add tests when there is enough logic to justify them.
- [ ] Add CI checks (formatting, clippy, build) on pull requests.
