# civ-ecs

Bevy ECS adapters for simulation data.

## Responsibility

- Mirror `civ-world` types as Bevy components
- Sync systems from `SimulationState` to ECS entities
- Marker components for renderer queries

## Separation

Simulation truth lives in `civ-simulation`. This crate only reflects it.
