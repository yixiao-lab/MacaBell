# Release Guide

## Local universal macOS build

Run this from the repository root on macOS:

```bash
./scripts/build-universal-macos.sh
```

The DMG will be generated under `src-tauri/target/universal-apple-darwin/release/bundle/dmg/`.

## GitHub Actions release

Push a tag:

```bash
git tag v0.2.0
git push origin v0.2.0
```

The workflow `.github/workflows/release.yml` will build a universal macOS DMG and create a draft GitHub Release.

## Code signing note

The current release workflow builds an unsigned DMG. This is okay for an early open-source project, but macOS may show a developer verification warning on first launch.

When the project has enough users, the next monetizable packaging step is a signed and notarized build.
