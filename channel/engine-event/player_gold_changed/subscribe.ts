import { onEngineEvent } from "@civ-channel/engine_event/runtime";
import type { PlayerGoldChanged } from "./model";

export const onPlayerGoldChanged = onEngineEvent<PlayerGoldChanged>("PlayerGoldChanged");
