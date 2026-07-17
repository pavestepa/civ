# civ-ui-bridge

IPC bridge between React (Layer 1) and Rust API (Layer 2).

## Protocol

```
React → IpcEnvelope → UiCommand → GameCommand → Simulation
Simulation → GameEvent → UiEvent → IpcEnvelope → React
```

## Channel

`UiChannel` uses crossbeam channels for Tauri ↔ Bevy communication.

## Rules

- UI never accesses ECS
- Renderer never knows about UI
