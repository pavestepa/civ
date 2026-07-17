import { EventLog } from "@/entities/event-log";
import { GoldStat } from "@/entities/gold";
import { TurnStat } from "@/entities/turn";
import { AddGoldButton } from "@/features/add-gold";
import { EndTurnButton } from "@/features/end-turn";
import { useEngineSync } from "../model/useEngineSync";

export function Hud() {
  useEngineSync();

  return (
    <div className="hud">
      <header className="hud-top">
        <TurnStat />
        <GoldStat />
      </header>

      <footer className="hud-bottom">
        <EndTurnButton />
        <AddGoldButton />
      </footer>

      <EventLog />
    </div>
  );
}
