# Civ

AAA-quality 3D turn-based strategy game foundation inspired by Civilization VI.

## Architecture

Four completely separated layers:

| Layer | Technology | Responsibility |
|-------|-----------|----------------|
| 1 — UI | React + TypeScript + Vite | Presentation only |
| 2 — API | Rust + Tauri + IPC | Bridge, owns game state |
| 3 — Engine | Bevy 0.15 (upgrade path to 0.19) | ECS, rendering, input, scheduling |
| 4 — Scripting | QuickJS | Mods, scenarios, DLC logic |

```
workspace/
├── engine/          # Bevy game binary
├── launcher/        # Tauri WebView host
├── ui/              # React overlay
├── crates/          # Modular library crates
└── assets/          # Game assets
```

## Requirements

- **Rust 1.88+** recommended (see `rust-toolchain.toml`)
- `civ-engine` builds with pinned Bevy 0.15
- `civ-launcher` requires Tauri 2 dependencies (Rust 1.88+)

## Quick Start

```bash
# Build engine
cargo run -p civ-engine

# UI development (separate terminal)
cd ui && npm install && npm run dev

# Launcher (requires UI dev server)
cargo run -p civ-launcher
```

## Design Principles

- **Simulation owns truth** — renderer observes ECS only
- **No custom renderer** — all rendering through Bevy PBR pipeline
- **Deterministic simulation** — independent of UI, renderer, QuickJS
- **Plugin architecture** — every subsystem is a Bevy `Plugin`
- **IPC-only UI** — React never touches ECS or Bevy

## Crates

See individual `README.md` files in each crate under `crates/`.

## Roadmap

See [TODO.md](TODO.md).
