# Asset Naming and Sizing Guide

## Goal

Keep asset intent obvious at a glance and prevent accidental misuse (for example, using a tiny icon where a large banner is expected).

## Naming Convention

`<name>-<width>x<height>.<ext>`

Examples:

- `crabcord-mark-56x56.svg`
- `crabcord-mark-120x120.svg`
- `crabcord-wordmark-360x72.svg`
- `crabcord-shell-1280x800.svg`
- `crabcord-shell-1280x800.png`

## Folder Layout

- `assets/brand/`
  - Logos, marks, wordmarks, icons.
- `assets/mock/`
  - Mockups and README screenshots.

## Current Asset Inventory

### `assets/brand/crabcord-mark-56x56.svg`

- Intended use: left rail/app icon tile.
- Used in app: not currently (reserved as vector mark variant).
- App path: `brand/crabcord-mark-56x56.svg`.

### `assets/brand/crabcord-mark-120x120.svg`

- Intended use: launcher/docs icon usage.
- Used in app: not yet (reserved).

### `assets/brand/crabcord-mascot-56x56.png`

- Intended use: left rail/app icon tile using the full mascot art.
- Used in app: yes.
- App path: `brand/crabcord-mascot-56x56.png`.

### `assets/brand/crabcord-mascot-120x120.png`

- Intended use: medium-size icon surfaces (profile/chip cards).
- Used in app: not yet (reserved).

### `assets/brand/crabcord-mascot-1024x1024.png`

- Intended use: canonical full-resolution raster source.
- Used in app: not directly (source asset).

### `assets/brand/crabcord-mascot-256x256.ico`

- Intended use: desktop/app-icon packaging target.
- Used in app runtime UI: no (packaging asset).

### `assets/brand/crabcord-wordmark-360x72.svg`

- Intended use: header wordmark.
- Used in app: yes.
- App path: `brand/crabcord-wordmark-360x72.svg`.

### `assets/mock/crabcord-shell-1280x800.svg`

- Intended use: README screenshot placeholder in headless environments.
- Used in app: not runtime UI, docs only.

### `assets/mock/crabcord-shell-1280x800.png`

- Intended use: current README screenshot captured from the running app window.
- Used in app: not runtime UI, docs only.

## Runtime Loading Path Rules

Assets are loaded from `assets/` using `FileAssetSource`:

- Full filesystem base: `<project>/assets`
- App runtime path example:
  - `brand/crabcord-mascot-56x56.png`

Do not include `assets/` in the runtime path string.

## Sizing Rules

- Use the closest-native asset size when possible.
- Avoid stretching square marks into non-square slots.
- Keep vector assets (`.svg`) as canonical source for scalable UI elements.
