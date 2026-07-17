import { Stat } from "@/shared/ui";
import { turnSelectors, useTurnStore } from "../model/store";

export function TurnStat() {
  const turn = useTurnStore(turnSelectors.turn);

  return <Stat label="Turn" value={turn} />;
}
