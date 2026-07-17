import { useCallback, useEffect, useState } from "react";
import { addGold } from "@channel/front-api/add_gold/call";
import { endTurn } from "@channel/front-api/end_turn/call";
import { onPlayerGoldChanged } from "@channel/engine-event/player_gold_changed/subscribe";
import { onTurnChanged } from "@channel/engine-event/turn_changed/subscribe";

export function Hud() {
  const [turn, setTurn] = useState(1);
  const [gold, setGold] = useState(0);
  const [log, setLog] = useState<string[]>([]);

  const pushLog = useCallback((entry: string) => {
    setLog((prev) => [entry, ...prev].slice(0, 8));
  }, []);

  useEffect(() => {
    const unsubscribeTurn = onTurnChanged(({ turn: nextTurn }) => {
      setTurn(nextTurn);
      pushLog(`TurnChanged: ${nextTurn}`);
    });

    const unsubscribeGold = onPlayerGoldChanged(({ gold: nextGold }) => {
      setGold(nextGold);
      pushLog(`PlayerGoldChanged: ${nextGold}`);
    });

    return () => {
      unsubscribeTurn();
      unsubscribeGold();
    };
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
        <button type="button" onClick={() => endTurn.call({})}>
          End Turn
        </button>
        <button type="button" onClick={() => addGold.call({ player: 0, amount: 10 })}>
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
