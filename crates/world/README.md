# civ-world

Renderer-independent world simulation data.

## Responsibility

- `WorldMap` aggregate state
- `Tile`, `Unit`, `City`, `Player` data structures
- Terrain, elevation, resources, rivers, roads, visibility, ownership

## Layer

Layer 3 simulation truth. The Bevy renderer observes ECS mirrors of this data.

## Rules

- No Bevy imports
- No UI imports
- No QuickJS imports
- Serializable with Serde for saves and network snapshots
