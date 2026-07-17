import { create } from "zustand";

const MAX_ENTRIES = 8;

type EventLogState = {
  entries: string[];
  push: (entry: string) => void;
  clear: () => void;
};

export const useEventLogStore = create<EventLogState>((set) => ({
  entries: [],
  push: (entry) =>
    set((state) => ({
      entries: [entry, ...state.entries].slice(0, MAX_ENTRIES),
    })),
  clear: () => set({ entries: [] }),
}));

export const eventLogSelectors = {
  entries: (state: EventLogState) => state.entries,
};
