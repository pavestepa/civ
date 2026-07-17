import { onPlayerGoldChanged } from "@/shared/api";
import { useGoldStore } from "./store";

export function subscribeGoldChanges(onChanged?: (gold: number) => void): () => void {
  return onPlayerGoldChanged(({ gold }) => {
    useGoldStore.getState().setGold(gold);
    onChanged?.(gold);
  });
}
