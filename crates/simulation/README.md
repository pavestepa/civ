# civ-simulation

Deterministic simulation layer owning world truth.

## Rules

- Never depends on React, renderer, or QuickJS
- Processes `GameCommand` events
- Emits `GameEvent` for observers

## Future

- Fixed-point math option for network lockstep
- Command log for replay
- Turn scheduler with player order
