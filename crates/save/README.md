# civ-save

Binary save/load using Postcard.

## Format

- `SaveGame` header with format version and game version
- Embedded `WorldMap` snapshot
- Checksum placeholder for future validation

## Future

- Incremental saves
- Save migration between format versions
- Replay-embedded command streams
