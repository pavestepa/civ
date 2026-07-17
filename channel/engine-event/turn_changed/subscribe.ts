import { onEngineEvent } from "@civ-channel/engine_event/runtime";
import type { TurnChanged } from "./model";

export const onTurnChanged = onEngineEvent<TurnChanged>("TurnChanged");
