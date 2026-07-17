import { useEffect } from "react";
import { subscribeGoldChanges } from "@/entities/gold";
import { subscribeTurnChanges } from "@/entities/turn";
import { useEventLogStore } from "@/entities/event-log";

export function useEngineSync() {
  useEffect(() => {
    const unsubscribeTurn = subscribeTurnChanges((turn) => {
      useEventLogStore.getState().push(`TurnChanged: ${turn}`);
    });

    const unsubscribeGold = subscribeGoldChanges((gold) => {
      useEventLogStore.getState().push(`PlayerGoldChanged: ${gold}`);
    });

    return () => {
      unsubscribeTurn();
      unsubscribeGold();
    };
  }, []);
}
