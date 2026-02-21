# Release and Packaging Notes

## Build Commands

Debug build:

```bash
cargo build
```

Release build:

```bash
cargo build --release
```

Binary output:

```text
target/release/crabcord
```

## Run Release Binary

```bash
./target/release/crabcord
```

## Minimal macOS `.app` Bundle (Manual)

```bash
mkdir -p dist/CrabCord.app/Contents/MacOS
cp target/release/crabcord dist/CrabCord.app/Contents/MacOS/CrabCord
```

Optional metadata file:

```bash
mkdir -p dist/CrabCord.app/Contents
cat > dist/CrabCord.app/Contents/Info.plist <<'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleName</key><string>CrabCord</string>
  <key>CFBundleDisplayName</key><string>CrabCord</string>
  <key>CFBundleExecutable</key><string>CrabCord</string>
  <key>CFBundleIdentifier</key><string>com.crabcord.app</string>
  <key>CFBundleVersion</key><string>0.1.0</string>
  <key>CFBundleShortVersionString</key><string>0.1.0</string>
  <key>LSMinimumSystemVersion</key><string>13.0</string>
</dict>
</plist>
PLIST
```

## Signing and Distribution (Later Stage)

- Add proper bundle assets/icons.
- Sign with Developer ID certificate.
- Notarize for macOS distribution.
- Add CI release pipeline to create artifacts on tags.

## Versioning Notes

- Keep semantic versions in `Cargo.toml`.
- Tag releases in git (`v0.1.0`, `v0.2.0`, ...).
- Include changelog entries for user-facing behavior changes.

## GitHub Release Flow

1. Ensure release build works locally:
```bash
cargo build --release
```

2. Commit changes and create a version tag:
```bash
git add .
git commit -m "Release v0.1.0"
git tag v0.1.0
```

3. Push branch and tag:
```bash
git push origin main
git push origin v0.1.0
```

4. Open GitHub Releases and create a release from the tag.
5. Attach build artifacts if distributing binaries.
