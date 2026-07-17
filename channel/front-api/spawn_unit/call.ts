import { defineApi } from "@civ-channel/front_api/runtime";
import type { SpawnUnitReq, SpawnUnitRes } from "./model";

export const spawnUnit = defineApi<"SpawnUnit", SpawnUnitReq, SpawnUnitRes>("SpawnUnit");
