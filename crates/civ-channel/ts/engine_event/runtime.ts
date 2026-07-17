import type { WireMessage } from "../wire";

declare global {
  interface Window {
    __civReceiveEvent?: (message: Extract<WireMessage, { kind: "evt" }>) => void;
  }
}

const eventListeners = new Map<string, Set<(body: unknown) => void>>();

function ensureEventHook() {
  window.__civReceiveEvent = (message) => {
    const listeners = eventListeners.get(message.op);
    if (!listeners) {
      return;
    }
    for (const listener of listeners) {
      listener(message.body);
    }
  };
}

ensureEventHook();

export function onEngineEvent<T>(op: string) {
  return (handler: (body: T) => void): (() => void) => {
    let listeners = eventListeners.get(op);
    if (!listeners) {
      listeners = new Set();
      eventListeners.set(op, listeners);
    }
    listeners.add(handler as (body: unknown) => void);
    return () => listeners?.delete(handler as (body: unknown) => void);
  };
}
