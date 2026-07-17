import { submitEndTurn } from "../api/endTurn";

export function EndTurnButton() {
  return (
    <button type="button" onClick={() => void submitEndTurn()}>
      End Turn
    </button>
  );
}
