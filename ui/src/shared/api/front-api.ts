export { addGold } from "@channel/front-api/add_gold/call";
export { endTurn } from "@channel/front-api/end_turn/call";
export { spawnUnit } from "@channel/front-api/spawn_unit/call";
export { createCity } from "@channel/front-api/create_city/call";
export { requestSnapshot } from "@channel/front-api/request_snapshot/call";

export type { AddGoldReq, AddGoldRes } from "@channel/front-api/add_gold/model";
export type { EndTurnReq, EndTurnRes } from "@channel/front-api/end_turn/model";
export type { SpawnUnitReq, SpawnUnitRes } from "@channel/front-api/spawn_unit/model";
export type { CreateCityReq, CreateCityRes } from "@channel/front-api/create_city/model";
export type {
  RequestSnapshotReq,
  RequestSnapshotRes,
} from "@channel/front-api/request_snapshot/model";
