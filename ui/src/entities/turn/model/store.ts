import { create } from "zustand";

type TurnState = {
  turn: number;
  setTurn: (turn: number) => void;
};

export const useTurnStore = create<TurnState>((set) => ({
  turn: 1,
  setTurn: (turn) => set({ turn }),
}));

export const turnSelectors = {
  turn: (state: TurnState) => state.turn,
};
