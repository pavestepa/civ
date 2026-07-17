import { endTurn } from "@/shared/api";

export async function submitEndTurn() {
  return endTurn.call({});
}
