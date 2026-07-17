import { useCallback, useEffect, useState } from "react";
import { dispatchCommand, pollEvents } from "../ipc";

export function Hud() {
  const [turn, setTurn] = useState(1);
  const [gold, setGold] = useState(0);
  const [log, setLog] = useState<string[]>([]);

  const pushLog = useCallback((entry: string) => {
    setLog((prev) => [entry, ...prev].slice(0, 8));
  }, []);

  useEffect(() => {
    const interval = setInterval(async () => {
      try {
        const events = await pollEvents();
        for (const raw of events) {
          pushLog(raw);
          if (raw.includes("TurnChanged")) {
            const parsed = JSON.parse(raw) as {
              payload?: { turn?: number; active_player?: number };
            };
            if (parsed.payload?.turn) setTurn(parsed.payload.turn);
          }
          if (raw.includes("PlayerGoldChanged")) {
            const parsed = JSON.parse(raw) as {
              payload?: { gold?: number };
            };
            if (parsed.payload?.gold !== undefined) setGold(parsed.payload.gold);
          }
        }
      } catch {
        // Engine not connected yet.
      }
    }, 500);
    return () => clearInterval(interval);
  }, [pushLog]);

  return (
    <div className="hud">
      <header className="hud-top">
        <div className="stat">
          <span className="label">Turn</span>
          <span className="value">{turn}</span>
        </div>
        <div className="stat">
          <span className="label">Gold</span>
          <span className="value">{gold}</span>
        </div>
      </header>

      <footer className="hud-bottom">
        <button type="button" onClick={() => dispatchCommand({ action: "EndTurn" })}>
          End Turn
        </button>
        <button
          type="button"
          onClick={() =>
            dispatchCommand({ action: "AddGold", data: { player: 0, amount: 10 } })
          }
        >
          +10 Gold
        </button>
      </footer>

      <aside className="hud-log">
        {log.map((entry, i) => (
          <div key={i} className="log-entry">
            {entry}
          </div>
        ))}
      </aside>
    </div>
  );
}
