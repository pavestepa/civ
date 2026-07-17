import { Stat } from "@/shared/ui";
import { goldSelectors, useGoldStore } from "../model/store";

export function GoldStat() {
  const gold = useGoldStore(goldSelectors.gold);

  return <Stat label="Gold" value={gold} />;
}
