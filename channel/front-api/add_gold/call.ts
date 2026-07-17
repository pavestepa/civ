import { defineApi } from "@civ-channel/front_api/runtime";
import type { AddGoldReq, AddGoldRes } from "./model";

export const addGold = defineApi<"AddGold", AddGoldReq, AddGoldRes>("AddGold");
