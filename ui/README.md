# civ-ui

React + TypeScript + Vite UI overlay.

## Responsibility

Presentation only. No game logic, simulation, or ECS access.

## IPC

All communication goes through Tauri commands defined in `civ-launcher`:

```
React → dispatch_command → Rust → UiCommand → Simulation
Simulation → UiEvent → poll_events → React
```

## Run

```bash
npm install
npm run dev
```
