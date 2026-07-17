import type { WireMessage } from "../wire";

export function postUiEvent(op: string, body: unknown) {
  if (!window.ipc?.postMessage) {
    return;
  }
  const message: WireMessage = { kind: "evt", op, body };
  window.ipc.postMessage(JSON.stringify(message));
}
