# CrabCord Asset Guide

## Goal

Keep the UI SVG-first while allowing pragmatic fallbacks where GPUI rendering or fidelity requires it.

## Format Policy

- Primary format: `.svg` for icons, badges, channel glyphs, and scalable UI art.
- Allowed raster exception: `assets/brand/crab.png` (used as a reliable in-app fallback for complex brand art).
- README screenshots live in `docs/screenshots/*.png` and are documentation assets, not runtime UI assets.
- Icon SVGs should prefer `currentColor` for theme-driven coloring.

## Folder Layout

- `assets/brand/`
  - brand marks, mascot vectors, wordmark, crab artwork files
- `assets/mock/`
  - historical mock assets and captured shell references
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

- `brand/crabcord-mark-56x56.svg`
- `brand/crab.png`
- `ui/icons/channels/channel-text.svg`
- `ui/badges/bot.svg`

## Current Usage Rules

- Keep interaction icons in SVG.
- Keep status/channel/nav glyphs in SVG.
- Use PNG fallback for complex crab artwork surfaces where needed.
- Keep screenshots out of runtime loading paths (docs only).
