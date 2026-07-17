# civ-launcher

Unified Civ application: **one process, one window**.

Bevy renders the 3D world. A transparent Tauri WebView (React HUD) is attached as a native child layer on top.

## Run

```bash
# From workspace root — starts Vite (debug) + Bevy + React overlay
cargo run -p civ-launcher
```

Prerequisites:

```bash
cd ui && npm install
```

## Architecture

```
cargo run -p civ-launcher
        │
        ├─ UiChannel::paired()  ─ shared IPC between UI and simulation
        ├─ Bevy (main winit loop, 3D renderer)
        └─ Tauri WebView child window (transparent React HUD)
              driven via App::run_iteration() each frame
```

## Standalone engine (no UI)

```bash
cargo run -p civ-engine
```

Useful for testing rendering without the overlay.

## Production build

```bash
cd ui && npm run build
cargo build -p civ-launcher --release
```
