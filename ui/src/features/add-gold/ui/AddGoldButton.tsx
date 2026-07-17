import { submitAddGold } from "../api/addGold";

export function AddGoldButton() {
  return (
    <button type="button" onClick={() => void submitAddGold()}>
      +10 Gold
    </button>
  );
}
