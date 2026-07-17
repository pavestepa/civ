import { defineApi } from "@civ-channel/front_api/runtime";
import type { EndTurnReq, EndTurnRes } from "./model";

export const endTurn = defineApi<"EndTurn", EndTurnReq, EndTurnRes>("EndTurn");
