declare global {
  interface Window {
    ipc?: { postMessage(message: string): void };
    __civReceiveEvents?: (events: string[]) => void;
  }
}

export type UiAction =
  | { action: "EndTurn" }
  | { action: "AddGold"; data: { player: number; amount: number } }
  | {
      action: "SpawnUnit";
      data: { owner: number; q: number; r: number; kind: string };
    }
  | {
      action: "CreateCity";
      data: { owner: number; q: number; r: number; name: string };
    }
  | { action: "RequestSnapshot" };

export async function dispatchCommand(action: UiAction): Promise<number> {
  if (!window.ipc) {
    throw new Error("UI overlay IPC is not available");
  }
  window.ipc.postMessage(JSON.stringify(action));
  return 0;
}

export function subscribeEvents(onEvents: (events: string[]) => void): () => void {
  const handler = (event: Event) => {
    const detail = (event as CustomEvent<string[]>).detail;
    if (Array.isArray(detail)) {
      onEvents(detail);
    }
  };
  window.addEventListener("civ:events", handler);
  return () => window.removeEventListener("civ:events", handler);
}
