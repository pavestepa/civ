# Civ — Architecture TODO

## Foundation (Complete)

- [x] Cargo workspace with 31 library crates
- [x] Engine, launcher, and UI app separation
- [x] Bevy plugin architecture
- [x] Simulation / render / UI layer separation
- [x] IPC protocol infrastructure
- [x] QuickJS binding infrastructure
- [x] Postcard save infrastructure
- [x] Example terrain, unit, city entities

## Engine & Rendering

- [ ] GPU instancing for terrain tiles (thousands of hexes)
- [ ] Frustum culling tuning for large maps
- [ ] GLTF unit and city models via Bevy asset pipeline
- [ ] LOD system for map entities
- [ ] Hex tile mesh batching
- [ ] Camera controller (pan, zoom, rotate)
- [ ] Selection highlighting
- [x] Shared IPC channel between launcher and engine (single process)
- [x] Single-window Bevy + transparent Tauri WebView overlay

## Simulation

- [ ] Full turn scheduler with player order
- [ ] Command log for replay system
- [ ] Fixed-point math option for network lockstep
- [ ] Yield and growth calculations
- [ ] District and wonder placement rules
- [ ] Population and housing model

## Gameplay Systems

- [ ] Combat resolution
- [ ] Movement costs and zone of control
- [ ] Economy and trade routes
- [ ] Technology tree
- [ ] Culture and borders
- [ ] Religion
- [ ] Diplomacy
- [ ] Governments and policies
- [ ] Great people
- [ ] Fog of war
- [ ] AI decision engine

## UI (React)

- [ ] Main menu and game setup screens
- [ ] City management panel
- [ ] Unit action panel
- [ ] Technology tree view
- [ ] Diplomacy screen
- [ ] Notification system
- [ ] Minimap
- [x] Shared IPC channel (single-process `UiChannel::paired`)

## Scripting (QuickJS)

- [ ] Full `rquickjs` class bindings for `game.*` API
- [ ] Script hot-reload for mod development
- [ ] Mod manifest format
- [ ] Scenario definition loader
- [ ] Event hook system (onTurnStart, onCityFounded, etc.)
- [ ] Sandboxed script execution

## Save & Network

- [ ] Save file versioning and migration
- [ ] Autosave
- [ ] Multiplayer command replication
- [ ] Dedicated headless server mode
- [ ] Replay playback from command log

## Platform & Distribution

- [ ] Steam Workshop integration
- [ ] Cross-platform build pipeline
- [ ] Map editor tool
- [ ] DLC content loading pipeline
- [ ] CI/CD for engine, launcher, and UI

## Developer Experience

- [ ] `cargo nextest` integration
- [ ] Bevy fast-compile profile documentation
- [ ] Crate-level integration tests
- [ ] Architecture decision records (ADRs)
