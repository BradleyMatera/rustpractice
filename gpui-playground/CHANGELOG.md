# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

- Rebuilt UI into a full multi-lane CrabCord shell (guild rail, channels/DMs, timeline, crew panel, asset desk).
- Added fake backend seed data for guilds, channels, DMs, members, and messages in `src/backend.rs`.
- Added backend command boundary (`BackendCommand` + `BackendSink`) and channel-backed worker wiring.
- Added optional Axiom compatibility path (`--features axiom-backend`).
- Converted runtime assets to PNG-only usage and archived legacy SVG sources locally.
- Updated screenshots, README, and all project docs to match current behavior.

## [0.1.0]

- Initial GPUI shell prototype.
- Local runtime asset loading from `assets/`.
- Interactive status controls (`Cycle Status`, `Try Stream Action`).
