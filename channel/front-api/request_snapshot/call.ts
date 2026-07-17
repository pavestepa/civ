import { defineApi } from "@civ-channel/front_api/runtime";
import type { RequestSnapshotReq, RequestSnapshotRes } from "./model";

export const requestSnapshot = defineApi<
  "RequestSnapshot",
  RequestSnapshotReq,
  RequestSnapshotRes
>("RequestSnapshot");
