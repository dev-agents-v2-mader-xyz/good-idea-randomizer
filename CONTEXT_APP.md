# CONTEXT_APP — good-idea-randomizer

## Status
Phase: app_done

## What was configured

### tauri.conf.json (crates/app/tauri.conf.json)
- productName: "Good Idea Randomizer"
- identifier: "xyz.mader.good-idea-randomizer"
- window: 800×600, title "Good Idea Randomizer"
- frontendDist: "../../dist" (Trunk builds to project root /dist/)
- CSP: "default-src 'self'; style-src 'self' 'unsafe-inline'"
- bundle.active: false (no bundling for now)

### capabilities/default.json
Already in repo. Contains `core:default` permission only — correct for a fully static app with no native APIs.

### icons/icon.png
Placeholder 32×32 RGBA PNG. Required by tauri::generate_context!(). Replace with real icon if desktop distribution is needed.

## Build order
1. `cd crates/frontend && trunk build --release --public-url /` → outputs to project root `/dist/`
2. `cargo build -p app` (requires dist/ to exist)

## Notes
- The app is a pure static web app; no Tauri plugins or deep links needed.
- The src-tauri/ subdirectory is a separate unused stub; the actual workspace crate is crates/app/ itself.
- Build verified: `cargo build -p app` succeeds (dev profile, native target).
