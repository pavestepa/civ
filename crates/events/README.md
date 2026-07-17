# civ-events

Event definitions for simulation, UI, and scripting bridges.

## Events

- `GameEvent` — simulation truth events
- `UiEvent` — IPC-facing presentation events

## Flow

```
Simulation → GameEvent → UiBridge → UiEvent → React
```
