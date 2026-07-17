# civ-quickjs

QuickJS integration for mods and scenarios.

## Bindings

Scripts may call:

- `game.spawnUnit()`
- `game.createCity()`
- `game.endTurn()`
- `game.addGold()`
- `game.getPlayer()`
- `game.findCity()`

## Rules

- Communicates only with Rust API (`civ-scripting`)
- Never talks to React or Bevy renderer
