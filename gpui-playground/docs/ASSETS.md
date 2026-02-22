# CrabCord Asset Guide

## Goal

Keep runtime assets simple and reliable: use PNG files for all UI assets.

## Format Policy

- Runtime format: `.png` for icons, badges, channel glyphs, avatars, and UI art.
- Brand variants: `assets/brand/crab-56.png`, `assets/brand/crab-96.png`, `assets/brand/crab-120.png` (derived from `assets/brand/crab.png`).
- README screenshots live in `docs/screenshots/*.png` and are documentation assets, not runtime UI assets.
- Legacy SVG source files are archived locally in `legacysvgs/` and ignored by git.

## Folder Layout

- `assets/brand/`
  - brand marks, mascot variants, wordmark, crab artwork files
- `assets/mock/`
  - current shell capture (`crabcord-shell-1280x800.png`) and local screenshot captures
- `assets/ui/icons/actions/`
  - send, invite, mute, edit, attach, etc.
- `assets/ui/icons/channels/`
  - text, voice, forum, announcements, etc.
- `assets/ui/icons/navigation/`
  - friends, discover, search, settings, etc.
- `assets/ui/icons/status/`
  - online, idle, dnd, offline, mobile, streaming
- `assets/ui/avatars/`
  - reusable avatar shapes
- `assets/ui/badges/`
  - bot/owner/mod/verified/etc.
- `assets/ui/illustrations/`
  - larger decorative UI elements
- `docs/screenshots/`
  - README screenshots (crew view + asset desk view)

## Runtime Paths

Runtime assets are loaded from `assets/` by `FileAssetSource`.

- Use paths relative to `assets/`
- Do not prefix with `assets/`

Examples:

- `brand/crabcord-mark-56x56.png`
- `brand/crab-56.png`
- `ui/icons/channels/channel-text.png`
- `ui/badges/bot.png`
- `mock/crabcord-shell-1280x800.png`

## Current Usage Rules

- Keep interaction icons in PNG.
- Keep status/channel/nav glyphs in PNG.
- Use small PNG brand variants for small logo/avatar surfaces.
- Keep screenshots out of runtime loading paths (docs only).
- Do not reference `legacysvgs/` paths from runtime code.
