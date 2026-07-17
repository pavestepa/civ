# civ-scripting

Scripting infrastructure layer.

## Scope

Mods, scenarios, civilization scripts, events, DLC logic.

## API

`ScriptGameApi` defines the Rust command surface scripts may invoke.
QuickJS bindings live in `civ-quickjs`.

## Rules

- Scripts communicate only with Rust API
- Never touch React or Bevy ECS directly
