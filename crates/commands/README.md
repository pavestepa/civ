# civ-commands

Command protocol for UI and scripting layers.

## Commands

- `UiCommand` — received from React via IPC
- `GameCommand` — dispatched to deterministic simulation

## Flow

```
React → UiCommand → Rust API → GameCommand → Simulation
```
