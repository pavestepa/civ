import { onTurnChanged } from "@/shared/api";
import { useTurnStore } from "./store";

export function subscribeTurnChanges(onChanged?: (turn: number) => void): () => void {
  return onTurnChanged(({ turn }) => {
    useTurnStore.getState().setTurn(turn);
    onChanged?.(turn);
  });
}
