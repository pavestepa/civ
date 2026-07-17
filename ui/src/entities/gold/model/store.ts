import { create } from "zustand";

type GoldState = {
  gold: number;
  setGold: (gold: number) => void;
};

export const useGoldStore = create<GoldState>((set) => ({
  gold: 0,
  setGold: (gold) => set({ gold }),
}));

export const goldSelectors = {
  gold: (state: GoldState) => state.gold,
};
