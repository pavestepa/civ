# civ-launcher

Tauri WebView host for the React UI overlay.

## Responsibility

- Host transparent React overlay
- Forward IPC commands to engine via `civ-ui-bridge`
- Launch or attach to Bevy renderer window

## Run

```bash
# Terminal 1 — engine
cargo run -p civ-engine

# Terminal 2 — launcher + UI
cargo run -p civ-launcher
```

## Layer

Layer 2 shell — bridge between React (Layer 1) and Rust API.
