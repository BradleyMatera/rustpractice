# SVG Asset Guide

## Goal

Maintain a single, consistent asset format (`.svg`) across brand, UI, and mock assets so scaling and theming stay predictable.

## Format Policy

- Allowed format in `assets/`: `.svg` only.
- No raster (`.png`, `.jpg`, `.webp`) and no icon container (`.ico`) files are kept in this repository.
- Icons should default to `currentColor` for flexible theming unless a fixed brand color is required.

## Folder Layout

- `assets/brand/`
  - Core brand marks, mascot vectors, wordmark.
- `assets/mock/`
  - SVG mock screenshots for docs.
- `assets/ui/icons/actions/`
  - Composer and interaction icons (`send`, `attach`, `mute`, `invite`, etc.).
- `assets/ui/icons/channels/`
  - Channel-type icons (`text`, `voice`, `forum`, `announcements`, etc.).
- `assets/ui/icons/navigation/`
  - App-level navigation icons (`home`, `discover`, `search`, etc.).
- `assets/ui/icons/status/`
  - Presence/status indicators (`online`, `idle`, `dnd`, etc.).
- `assets/ui/avatars/`
  - Reusable avatar SVG variants.
- `assets/ui/badges/`
  - Badges (`bot`, `verified`, `owner`, `mod`, etc.).
- `assets/ui/illustrations/`
  - Large UI illustrations (`server-banner`, empty states, onboarding).

## Current Inventory

- Total files: 65 SVG assets.
- Brand SVGs: 6
- Mock SVGs: 1
- UI SVGs: 58

To list all assets:

```bash
find assets -type f | sort
```

To enforce SVG-only assets:

```bash
find assets -type f ! -name '*.svg'
```

The command above should return no results.

## Runtime Loading Paths

Assets are loaded by `FileAssetSource` from the `assets/` base directory.

- Runtime path includes subfolder under `assets/`.
- Do not prefix runtime paths with `assets/`.

Examples:

- `brand/crabcord-wordmark-360x72.svg`
- `ui/icons/channels/channel-text.svg`
- `ui/badges/verified.svg`
- `mock/crabcord-shell-1280x800.svg`

## Usage Rules

- Use SVGs for all UI icon surfaces (guild rail, channels, members, controls).
- Use brand SVGs for identity surfaces (logo, wordmark, mascot marks).
- Keep illustrations in `assets/ui/illustrations/`; do not mix with icons.
- Keep icon viewboxes consistent for each category (`24x24` for most UI icons, `16x16` for status dots).
