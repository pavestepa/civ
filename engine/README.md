# civ-engine

Bevy game engine binary.

## Run

```bash
cargo run -p civ-engine
```

## Architecture

This crate wires all subsystem plugins. It contains no gameplay logic.

```
main.rs → app::run() → EnginePluginGroup
```

## Asset path

Set `BEVY_ASSET_ROOT` or run from workspace root so `assets/` resolves correctly.
