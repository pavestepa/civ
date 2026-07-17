import { defineApi } from "@civ-channel/front_api/runtime";
import type { CreateCityReq, CreateCityRes } from "./model";

export const createCity = defineApi<"CreateCity", CreateCityReq, CreateCityRes>("CreateCity");
