import type { WireMessage } from "../wire";

type PendingRequest = {
  op: string;
  resolve: (body: unknown) => void;
  reject: (error: Error) => void;
  timer: ReturnType<typeof setTimeout>;
};

const REQUEST_TIMEOUT_MS = 10_000;
const pendingRequests = new Map<number, PendingRequest>();
let nextRequestId = 1;

declare global {
  interface Window {
    __civReceiveResponse?: (message: Extract<WireMessage, { kind: "res" }>) => void;
    ipc?: {
      postMessage: (message: string) => void;
    };
  }
}

function ensureResponseHook() {
  window.__civReceiveResponse = (message) => {
    const pending = pendingRequests.get(message.id);
    if (!pending || pending.op !== message.op) {
      return;
    }
    clearTimeout(pending.timer);
    pendingRequests.delete(message.id);
    pending.resolve(message.body);
  };
}

function postRequest(message: WireMessage) {
  if (!window.ipc?.postMessage) {
    throw new Error("wry IPC bridge is not available");
  }
  window.ipc.postMessage(JSON.stringify(message));
}

ensureResponseHook();

export function defineApi<Op extends string, Req, Res>(op: Op) {
  return {
    op,
    async call(req: Req): Promise<Res> {
      const id = nextRequestId++;
      const message: WireMessage = { kind: "req", op, id, body: req };

      return new Promise<Res>((resolve, reject) => {
        const timer = setTimeout(() => {
          pendingRequests.delete(id);
          reject(new Error(`Request timed out: ${op}#${id}`));
        }, REQUEST_TIMEOUT_MS);

        pendingRequests.set(id, {
          op,
          resolve: (body) => resolve(body as Res),
          reject,
          timer,
        });

        try {
          postRequest(message);
        } catch (error: unknown) {
          clearTimeout(timer);
          pendingRequests.delete(id);
          reject(error instanceof Error ? error : new Error(String(error)));
        }
      });
    },
  };
}
