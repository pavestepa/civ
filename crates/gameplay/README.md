# civ-gameplay

Gameplay orchestration layer.

## Responsibility

- Application state machine (`AppState`)
- Session resources
- Coordinates subsystem plugins

## Subsystems

Each gameplay domain (combat, economy, research, etc.) is a separate plugin crate.
